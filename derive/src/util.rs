pub fn struct_field(field: &syn::Field, index: u32) -> syn::Expr {
    field
        .ident
        .as_ref()
        .map(|ident| syn::parse_quote!(self.#ident))
        .unwrap_or_else(|| {
            let index = syn::Index {
                index,
                span: proc_macro2::Span::mixed_site(),
            };
            syn::parse_quote! {self.#index}
        })
}
