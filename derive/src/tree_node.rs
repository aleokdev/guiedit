use proc_macro::TokenStream;
use quote::{quote, ToTokens};
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

        // TODO: TreeNode for Enum
        syn::Data::Enum(_enum) => {
            todo!()
        }
        syn::Data::Union(_union) => panic!("Unions are currently not supported as inspectables"),
    }
}

fn derive_struct(
    r#struct: syn::DataStruct,
    generics: &syn::Generics,
    ident: &proc_macro2::Ident,
    where_clause: &Option<syn::WhereClause>,
) -> TokenStream {
    let tree_node = usages::tree_node_trait();
    let inspectable = usages::inspectable_trait();
    let ui = usages::egui_ui();

    let mut tree_node_specialization = Specialization::new();
    tree_node_specialization.default_case(
        tree_node.clone(),
        quote! {
            fn inspect_child(&mut self, _: u64, _: u64, _: &mut #ui) {}

            fn node_ui(&mut self, _: &str, _: u64, _: &mut Option<u64>, _: &mut #ui) {}
        },
    )
    // Implementation for fields represented as nodes in the tree (Those that implement TreeNode / Inspectable)
    // Just forward the impl to the T itself
    .add_case_for_bounds(syn::parse_quote!(#tree_node), quote! {
        fn inspect_child(&mut self, this_id: u64, search_id: u64, ui: &mut #ui) {
            self.0.0.inspect_child(this_id, search_id, ui)
        }

        fn node_ui(&mut self, name: &str, id: u64, selected: &mut Option<u64>, ui: &mut #ui)  {
            self.0.0.node_ui(name, id, selected, ui)
        }

        fn contents_ui(&mut self, id: u64, selected: &mut Option<u64>, ui: &mut #ui) {
            self.0.0.contents_ui(id, selected, ui)
        }
    });
    tree_node_specialization
        .default_case(
            inspectable.clone(),
            quote! {
                fn inspect_ui(&mut self, ui: &mut #ui) {}
            },
        )
        .add_case_for_bounds(
            syn::parse_quote!(#inspectable),
            quote! {
                fn inspect_ui(&mut self, ui: &mut #ui) {
                    self.0.0.inspect_ui(ui);
                }
            },
        );
    let tree_node_specialization = tree_node_specialization.build();
    let wrap_tree_elements_impl = quote! {
        #tree_node_specialization

        use ::std::hash::Hasher;
        let mut hasher = std::collections::hash_map::DefaultHasher::default();
        hasher.write_u64(id);
    };
    let fields_tree_ui = r#struct.fields.iter().enumerate().fold(
        wrap_tree_elements_impl.clone(),
        |tokens, (idx, field)| {
            let name = field
                .ident
                .as_ref()
                .map(|ident| ident.to_string())
                .unwrap_or(idx.to_string());
            let field = util::struct_field(&field, idx as u32);
            quote! {
                #tokens
                hasher.write_u64(0);
                Wrap(Wrap(&mut #field)).node_ui(#name, hasher.clone().finish(), selected, ui);
            }
        },
    );
    let fields_search =
        r#struct
            .fields
            .iter()
            .enumerate()
            .fold(wrap_tree_elements_impl, |tokens, (idx, field)| {
                let field = util::struct_field(&field, idx as u32);
                quote! {
                    #tokens
                    hasher.write_u64(0);
                    Wrap(Wrap(&mut #field)).inspect_child(hasher.clone().finish(), search_id, ui);
                }
            });
    quote! {
        #[automatically_derived]
        impl #generics #tree_node for #ident #generics #where_clause {
            fn inspect_child(&mut self, id: u64, search_id: u64, ui: &mut #ui) {
                use #inspectable;
                use #tree_node;

                if id == search_id {
                    self.inspect_ui(ui);
                } else {
                    #fields_search
                }
            }

            fn contents_ui(&mut self, id: u64, selected: &mut Option<u64>, ui: &mut #ui) {
                use #inspectable;
                use #tree_node;

                #fields_tree_ui
            }
        }
    }
    .into()
}
