#[inline]
pub fn inspectable_trait() -> syn::TypePath {
    syn::parse_quote!(::guiedit::inspectable::Inspectable)
}

#[inline]
pub fn tree_node_trait() -> syn::TypePath {
    syn::parse_quote!(::guiedit::tree::TreeNode)
}

#[inline]
pub fn egui_ui() -> syn::TypePath {
    syn::parse_quote!(::guiedit::egui::Ui)
}
