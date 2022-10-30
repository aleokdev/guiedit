use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let where_clause = &generics.where_clause;

    match data {
        syn::Data::Struct(r#struct) => {
            let where_clause_init = if let Some(clause) = where_clause {
                quote! { #clause }
            } else if r#struct.fields.len() > 0 {
                quote! { where }
            } else {
                quote! {}
            };

            // Impose restriction that all fields must also implement Inspectable
            let where_clause = r#struct
                .fields
                .iter()
                .fold(where_clause_init, |clause, field| {
                    let field_ty = &field.ty;

                    quote! { #clause #field_ty: guiedit::inspectable::Inspectable, }
                });

            let fields_ui = r#struct.fields.iter().enumerate().fold(
                proc_macro2::TokenStream::new(),
                |tokens, (idx, field)| {
                    let field_ident = &field.ident;
                    let name = field
                        .ident
                        .as_ref()
                        .map(|ident| ident.to_string())
                        .unwrap_or(idx.to_string());
                    quote! {
                        #tokens
                        ui.horizontal(|ui| {
                            ui.label(#name);
                            self.#field_ident.inspect_ui(ui);
                        });
                    }
                },
            );

            quote! {
                #[automatically_derived]
                impl #generics guiedit::inspectable::Inspectable for #ident #generics #where_clause {
                    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
                        ui.group(|ui| {
                            ui.label(stringify!(#ident));
                            #fields_ui
                        });
                    }
                }
            }
            .into()
        }

        syn::Data::Enum(r#enum) => {
            let checkbox_variants_ui =
                r#enum
                    .variants
                    .iter()
                    .fold(proc_macro2::TokenStream::new(), |tokens, variant| {
                        let variant_ident = &variant.ident;
                        let variant_default = match variant.fields.iter().nth(0) {
                            Some(syn::Field{ident: None, ..}) => {
                                // Tuple variant
                                let fields = variant.fields.iter().fold(proc_macro2::TokenStream::new(), |tokens, _| {
                                    quote! {
                                        #tokens
                                        Default::default(),
                                    }
                                });
                                quote! {
                                    Self:: #variant_ident ( #fields )
                                }
                            },

                            Some(syn::Field{ident: Some(_), ..}) => {
                                // Named variant
                                let fields = variant.fields.iter().fold(proc_macro2::TokenStream::new(), |tokens, field| {
                                    let field_ident = field.ident.as_ref().unwrap();
                                    quote! {
                                        #tokens
                                        #field_ident: Default::default(),
                                    }
                                });
                                quote! {
                                    Self:: #variant_ident { #fields }
                                }
                            },

                            None => {
                                // Unit variant
                                quote!{ Self:: #variant_ident }
                            },
                        };
                        quote! {
                            #tokens
                            let mut response = ui.add(egui::SelectableLabel::new(matches!(self, Self:: #variant_ident{..}), stringify!(#variant_ident)));
                            if response.clicked() {
                                selected_text = stringify!(#variant_ident).to_owned();
                                *self = #variant_default;
                                response.mark_changed();
                            }
                        }
                    });

            let fields_ui =
                r#enum
                    .variants
                    .iter()
                    .fold(proc_macro2::TokenStream::new(), |tokens, variant| {
                        let variant_ident = &variant.ident;
                        let mut is_tuple_enum = false;
                        let (fields, ui) = variant.fields.iter().enumerate().fold(
                            (
                                proc_macro2::TokenStream::new(),
                                proc_macro2::TokenStream::new(),
                            ),
                            |(f_tokens, ui_tokens), (idx, field)| {
                                let name = format!("_{}", idx);
                                let field_ident = &field
                                    .ident
                                    .as_ref()
                                    .map(|ident| quote! { #ident })
                                    .unwrap_or_else(|| {
                                        is_tuple_enum = true;
                                        let name =
                                            syn::Ident::new(&name, proc_macro2::Span::mixed_site());
                                        quote! { #name }
                                    });
                                let name = field
                                    .ident
                                    .as_ref()
                                    .map(|ident| ident.to_string())
                                    .unwrap_or(idx.to_string());
                                (
                                    quote! {
                                        #f_tokens #field_ident,
                                    },
                                    quote! {
                                        #ui_tokens
                                        ui.horizontal(|ui| {
                                            ui.label(#name);
                                            #field_ident.inspect_ui(ui);
                                        });
                                    },
                                )
                            },
                        );

                        if is_tuple_enum {
                            quote!(
                                #tokens
                                Self:: #variant_ident ( #fields ) => { #ui },
                            )
                        } else {
                            quote!(
                                #tokens
                                Self:: #variant_ident { #fields } => { #ui },
                            )
                        }
                    });

            quote! {
                #[automatically_derived]
                impl #generics guiedit::inspectable::Inspectable for #ident #generics #where_clause {
                    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
                        ui.group(|ui| {
                            ui.label(stringify!(#ident));

                            let mut selected_text = String::new();
                            egui::ComboBox::from_label("Variant")
                                .selected_text(&selected_text)
                                .show_ui(ui, |ui| {
                                    #checkbox_variants_ui
                                }
                            );

                            match self {
                                #fields_ui
                            }
                        });
                    }
                }
            }
            .into()
        }
        syn::Data::Union(_union) => panic!("Unions are currently not supported as inspectables"),
    }
}
