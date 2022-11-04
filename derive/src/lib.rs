use proc_macro::TokenStream;

mod inspectable;
mod tree_element;

#[proc_macro_derive(Inspectable)]
pub fn derive_inspectable(input: TokenStream) -> TokenStream {
    inspectable::derive(input)
}

#[proc_macro_derive(TreeElement)]
pub fn derive_tree_element(input: TokenStream) -> TokenStream {
    tree_element::derive(input)
}
