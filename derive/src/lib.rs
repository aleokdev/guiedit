use proc_macro::TokenStream;

mod inspectable;

#[proc_macro_derive(Inspectable)]
pub fn derive_inspectable(input: TokenStream) -> TokenStream {
    inspectable::derive(input)
}
