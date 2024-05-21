use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn imports(_input: TokenStream) -> TokenStream {
    quote! {
        include!(concat!(env!("OUT_DIR"), "/adder.rs"));
        type Adder = library::Adder<AdderType>;
    }.into()
}

#[proc_macro]
pub fn api_type(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}