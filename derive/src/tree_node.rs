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
            let wrap_tree_elements_impl = quote! {
                struct DerefWrap<T>(T);
                struct Wrap<T>(T);

                impl<T> std::ops::Deref for DerefWrap<T> {
                    type Target = T;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }

                impl<T> std::ops::DerefMut for DerefWrap<T> {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.0
                    }
                }

                // Implementation for fields represented as nodes in the tree (Those that implement TreeNode / Inspectable)
                // Just forward the impl to the T itself
                impl<T: Inspectable> Inspectable for DerefWrap<Wrap<&mut T>> {
                    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
                        self.0.0.inspect_ui(ui);
                    }
                }
                impl<T: TreeNode> TreeNode for DerefWrap<Wrap<&mut T>> {
                    fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut egui::Ui) {
                        self.0.0.inspect_child(this_id, search_id, ui)
                    }

                    fn tree_ui_outside(&mut self, name: &str, id: u64, selected: &mut Option<u64>, ui: &mut egui::Ui)  {
                        self.0.0.tree_ui_outside(name, id, selected, ui)
                    }

                    fn tree_ui(&mut self, id: u64, selected: &mut Option<u64>, ui: &mut egui::Ui) {
                        self.0.0.tree_ui(id, selected, ui)
                    }
                }

                // Implementation for fields not represented as nodes in the tree (Those that don't implement TreeNode)
                impl<T: Inspectable> Inspectable for Wrap<T> {
                    fn inspect_ui(&mut self, ui: &mut egui::Ui) {
                        self.0.inspect_ui(ui);
                    }
                }
                impl<T: Inspectable> TreeNode for Wrap<T> {
                    fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut egui::Ui) {
                        (this_id == search_id).then(|| self.0.inspect_ui(ui));
                    }

                    fn tree_ui_outside(&mut self, _: &str, _: u64, _: &mut Option<u64>, _: &mut egui::Ui) { }
                }

                use ::std::hash::Hasher;
                let mut hasher = std::collections::hash_map::DefaultHasher::default();
                hasher.write_u64(id);
            };

            let fields_tree_ui = r#struct.fields.iter().enumerate().fold(
                wrap_tree_elements_impl.clone(),
                |tokens, (idx, field)| {
                    let field_ident = &field.ident;
                    let name = field
                        .ident
                        .as_ref()
                        .map(|ident| ident.to_string())
                        .unwrap_or(idx.to_string());
                    quote! {
                        #tokens
                        hasher.write_u64(0);
                        DerefWrap(Wrap(&mut self.#field_ident)).tree_ui_outside(#name, hasher.clone().finish(), selected, ui);
                    }
                },
            );
            let fields_search = r#struct.fields.iter().fold(
                wrap_tree_elements_impl,
                |tokens, field| {
                    let field_ident = &field.ident;
                    quote! {
                        #tokens
                        hasher.write_u64(0);
                        DerefWrap(Wrap(&mut self.#field_ident)).inspect_child(hasher.clone().finish(), search_id, ui);
                    }
                },
            );

            quote! {
                #[automatically_derived]
                impl #generics ::guiedit::tree::TreeNode for #ident #generics #where_clause {
                    fn inspect_child(&mut self, id: u64, search_id: u64, ui: &mut egui::Ui) {
                        use ::guiedit::inspectable::Inspectable;
                        use ::guiedit::tree::TreeNode;

                        if id == search_id {
                            self.inspect_ui(ui);
                        } else {
                            #fields_search
                        }
                    }

                    fn tree_ui(&mut self, id: u64, selected: &mut Option<u64>, ui: &mut egui::Ui) {
                        use ::guiedit::inspectable::Inspectable;
                        use ::guiedit::tree::TreeNode;

                        #fields_tree_ui
                    }
                }
            }
            .into()
        }

        // TODO: TreeNode for Enum
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
                            let mut response = ui.add(guiedit::egui::SelectableLabel::new(matches!(self, Self:: #variant_ident{..}), stringify!(#variant_ident)));
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
                                            #field_ident.tree_ui(ui);
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
                impl #generics TreeNode for #ident #generics #where_clause {
                    fn tree_ui(&mut self, ui: &mut guiedit::egui::Ui) {
                        // TODO
                        ui.group(|ui| {
                            ui.label(stringify!(#ident));

                            let mut selected_text = String::new();
                            guiedit::egui::ComboBox::from_label("Variant")
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
