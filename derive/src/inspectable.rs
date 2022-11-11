use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::{specialization::Specialization, usages, util};

pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let where_clause = &generics.where_clause;

    match data {
        syn::Data::Struct(r#struct) => derive_struct(r#struct, &generics, &ident, where_clause),

        syn::Data::Enum(r#enum) => derive_enum(r#enum, &generics, &ident, where_clause),
        syn::Data::Union(_union) => panic!("Unions are currently not supported as inspectables"),
    }
}

fn derive_enum(
    r#enum: syn::DataEnum,
    generics: &syn::Generics,
    ident: &proc_macro2::Ident,
    where_clause: &Option<syn::WhereClause>,
) -> TokenStream {
    let inspectable = usages::inspectable_trait();
    let ui = usages::egui_ui();

    let mut specialization = Specialization::new();
    specialization
        .default_case(
            inspectable.clone(),
            quote! {
                fn inspect_ui(&mut self, ui: &mut #ui) {
                    ui.add_enabled_ui(false, |ui| ui.label("Does not implement Inspectable"));
                }
            },
        )
        .add_case_for_bounds(
            syn::parse_quote!(#inspectable),
            quote! {
                fn inspect_ui_outside(&mut self, name: &str, ui: &mut #ui) {
                    self.0.0.inspect_ui_outside(name, ui);
                }

                fn inspect_ui(&mut self, ui: &mut #ui) {
                    self.0.0.inspect_ui(ui);
                }
            },
        );

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
                    let mut response = ui.add(::guiedit::egui::SelectableLabel::new(matches!(self, Self:: #variant_ident{..}), stringify!(#variant_ident)));
                    if response.clicked() {
                        *self = #variant_default;
                        response.mark_changed();
                    }
                }
            });
    let variants_ui =
        r#enum
            .variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |tokens, variant| {
                let variant_ident = &variant.ident;
                let (destructure, fields_idents) = util::destructure_fields(
                    {
                        let path: syn::Path = syn::parse_quote! { #ident::#variant_ident };
                        &syn::PatPath {
                            path,
                            attrs: vec![],
                            qself: None,
                        }
                    },
                    &variant.fields,
                );

                let ui = inspect_ui_for_idents(ident, fields_idents.iter());

                quote!(
                    #tokens
                    #destructure => { #ui },
                )
            });

    let selected_text_patterns =
        r#enum
            .variants
            .iter()
            .fold(proc_macro2::TokenStream::new(), |tokens, variant| {
                let variant_ident = &variant.ident;

                quote! {
                    #tokens
                    #ident::#variant_ident {..} => stringify!(#variant_ident),
                }
            });

    quote! {
        #[automatically_derived]
        impl #generics #inspectable for #ident #generics #where_clause {
            fn inspect_ui(&mut self, ui: &mut #ui) {
                ui.group(|ui| {
                    ui.label(stringify!(#ident));

                    ::guiedit::egui::ComboBox::from_label("Variant")
                        .selected_text(match &self { #selected_text_patterns })
                        .show_ui(ui, |ui| {
                            #checkbox_variants_ui
                        }
                    );

                    match self {
                        #variants_ui
                    }
                });
            }
        }
    }
    .into()
}

fn derive_struct(
    syn::DataStruct { fields, .. }: syn::DataStruct,
    generics: &syn::Generics,
    ident: &proc_macro2::Ident,
    where_clause: &Option<syn::WhereClause>,
) -> TokenStream {
    let inspectable = usages::inspectable_trait();
    let ui = usages::egui_ui();

    let (destructure_pat, fields_idents) = util::destructure_fields(
        {
            let path: syn::Path = syn::parse_quote!( #ident );
            &syn::PatPath {
                path,
                attrs: vec![],
                qself: None,
            }
        },
        &fields,
    );

    let fields_inspect_ui = inspect_ui_for_idents(ident, fields_idents.iter());
    quote! {
        #[automatically_derived]
        impl #generics #inspectable for #ident #generics #where_clause {
            fn inspect_ui_outside(&mut self, _name: &str, _ui: &mut #ui) {}

            fn inspect_ui(&mut self, ui: &mut #ui) {
                let #destructure_pat = &mut *self;
                #fields_inspect_ui
            }
        }
    }
    .into()
}

fn inspect_ui_for_idents<'i, IdentIter>(
    parent_ident: &proc_macro2::Ident,
    idents: IdentIter,
) -> proc_macro2::TokenStream
where
    IdentIter: Iterator<Item = &'i syn::Ident>,
{
    let inspectable = usages::inspectable_trait();
    let ui = usages::egui_ui();

    let mut specialization = Specialization::new();
    specialization
        .default_case(
            inspectable.clone(),
            quote! {
                fn inspect_ui(&mut self, ui: &mut #ui) {
                    ui.add_enabled_ui(false, |ui| ui.label("Does not implement Inspectable"));
                }
            },
        )
        .add_case_for_bounds(
            syn::parse_quote!(#inspectable),
            quote! {
                fn inspect_ui_outside(&mut self, name: &str, ui: &mut #ui) {
                    self.0.0.inspect_ui_outside(name, ui);
                }

                fn inspect_ui(&mut self, ui: &mut #ui) {
                    self.0.0.inspect_ui(ui);
                }
            },
        );
    let specialization = specialization.build();
    let fields_inspect_ui = idents.fold(specialization, |tokens, ident| {
        quote! {
            #tokens
            Wrap(Wrap(#ident)).inspect_ui_outside(stringify!(#ident), ui);
        }
    });

    quote! {
        use #inspectable;

        ui.group(|ui| {
            ui.label(stringify!(#parent_ident));
            #fields_inspect_ui
        });
    }
}
