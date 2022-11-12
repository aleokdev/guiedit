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
///
/// # Important
/// Ignores fields marked with the `inspectable(ignore)` attribute.
pub fn destructure_fields(path: syn::Path, fields: &syn::Fields) -> (syn::Pat, Vec<Ident>) {
    fn filter_ignored<'f, 'i>(
        x: impl Iterator<Item = &'f syn::Field> + Clone + 'i,
    ) -> impl Iterator<Item = &'f syn::Field> + Clone + 'i {
        x.filter(|field| {
            !field.attrs.iter().any(|attr| {
                attr.path
                    .get_ident()
                    .map_or(false, |ident| &ident.to_string() == "inspectable")
                    && &attr.tokens.to_string() == "(ignore)"
            })
        })
    }

    match fields {
        syn::Fields::Named(fields) => {
            let idents = filter_ignored(fields.named.iter())
                .map(|field| field.ident.as_ref().cloned().unwrap());
            let fields = idents
                .clone()
                .map(|ident| syn::FieldPat {
                    member: syn::Member::Named(ident.clone()),
                    colon_token: None,
                    attrs: vec![],
                    pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                        ident,
                        mutability: Some(syn::token::Mut {
                            span: proc_macro2::Span::mixed_site(),
                        }),
                        attrs: vec![],
                        by_ref: Some(syn::token::Ref {
                            span: proc_macro2::Span::mixed_site(),
                        }),
                        subpat: None,
                    })),
                })
                .collect::<syn::punctuated::Punctuated<syn::FieldPat, syn::Token![,]>>();

            let pat = syn::Pat::Struct(syn::PatStruct {
                attrs: vec![],
                brace_token: syn::token::Brace::default(),
                fields,
                dot2_token: Some(syn::token::Dot2::default()),
                path,
            });

            (pat, idents.into_iter().collect())
        }
        syn::Fields::Unnamed(fields) => {
            let idents = filter_ignored(fields.unnamed.iter())
                .enumerate()
                .map(|(idx, _)| {
                    // I actually have no idea how this compiles. This string is decided at runtime
                    // yet I can take a static reference to it. Huh??
                    let name = format!("_{}", idx);
                    syn::Ident::new(&name, proc_macro2::Span::mixed_site())
                });
            let fields = idents
                .clone()
                .map(|ident| {
                    syn::PatIdent {
                        ident,
                        mutability: Some(syn::token::Mut {
                            span: proc_macro2::Span::mixed_site(),
                        }),
                        attrs: vec![],
                        by_ref: Some(syn::token::Ref {
                            span: proc_macro2::Span::mixed_site(),
                        }),
                        subpat: None,
                    }
                    .into()
                })
                .chain(std::iter::once::<syn::Pat>(
                    syn::PatRest {
                        attrs: vec![],
                        dot2_token: syn::token::Dot2::default(),
                    }
                    .into(),
                ))
                .collect::<syn::punctuated::Punctuated<syn::Pat, syn::Token![,]>>();

            let pat = syn::Pat::TupleStruct(syn::PatTupleStruct {
                path,
                pat: syn::PatTuple {
                    attrs: vec![],
                    paren_token: syn::token::Paren::default(),
                    elems: fields,
                },
                attrs: vec![],
            });
            (pat, idents.into_iter().collect())
        }
        syn::Fields::Unit => (
            syn::Pat::Path(syn::PatPath {
                attrs: vec![],
                path,
                qself: None,
            }),
            vec![],
        ),
    }
}
