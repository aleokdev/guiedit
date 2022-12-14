use proc_macro::TokenStream;

mod inspectable;
mod specialization;
mod tree_node;
mod usages;
mod util;

#[proc_macro_derive(Inspectable, attributes(inspectable))]
pub fn derive_inspectable(input: TokenStream) -> TokenStream {
    inspectable::derive(input)
}

#[proc_macro_derive(TreeNode)]
pub fn derive_tree_node(input: TokenStream) -> TokenStream {
    tree_node::derive(input)
}
