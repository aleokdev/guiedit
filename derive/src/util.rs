use proc_macro2::Ident;

pub fn struct_field(field_ident: Option<&syn::Ident>, index: u32) -> syn::Expr {
    field_ident
        .map(|ident| syn::parse_quote!(self.#ident))
        .unwrap_or_else(|| {
            let index = syn::Index {
                index,
                span: proc_macro2::Span::mixed_site(),
            };
            syn::parse_quote! {self.#index}
        })
}

/// Generates a pattern destructuring all fields given, along with the identifiers bound to them.
///
/// Generates code equivalent to:
/// ```ignore
/// Path { field0: mut field0_bound, field1: mut field1_bound, ... };
/// ```
pub fn destructure_fields(path: &syn::PatPath, fields: &syn::Fields) -> (syn::Pat, Vec<Ident>) {
    match fields {
        syn::Fields::Named(fields) => {
            let idents = fields
                .named
                .iter()
                .map(|field| field.ident.as_ref().cloned().unwrap());
            let fields = idents
                .clone()
                .map(|ident| syn::PatIdent {
                    ident: ident,
                    mutability: Some(syn::token::Mut {
                        span: proc_macro2::Span::mixed_site(),
                    }),
                    attrs: vec![],
                    by_ref: Some(syn::token::Ref {
                        span: proc_macro2::Span::mixed_site(),
                    }),
                    subpat: None,
                })
                .collect::<syn::punctuated::Punctuated<syn::PatIdent, syn::Token![,]>>();
            (
                syn::parse_quote! { #path { #fields } },
                idents.into_iter().collect(),
            )
        }
        syn::Fields::Unnamed(fields) => {
            let idents = fields.unnamed.iter().enumerate().map(|(idx, _)| {
                // I actually have no idea how this compiles. This string is decided at runtime
                // yet I can take a static reference to it. Huh??
                let name = format!("_{}", idx);
                syn::Ident::new(&name, proc_macro2::Span::mixed_site())
            });
            let fields = idents
                .clone()
                .map(|ident| syn::PatIdent {
                    ident,
                    mutability: Some(syn::token::Mut {
                        span: proc_macro2::Span::mixed_site(),
                    }),
                    attrs: vec![],
                    by_ref: Some(syn::token::Ref {
                        span: proc_macro2::Span::mixed_site(),
                    }),
                    subpat: None,
                })
                .collect::<syn::punctuated::Punctuated<syn::PatIdent, syn::Token![,]>>();

            (
                syn::parse_quote! { #path ( #fields ) },
                idents.into_iter().collect(),
            )
        }
        syn::Fields::Unit => (syn::parse_quote! ( #path ), vec![]),
    }
}
