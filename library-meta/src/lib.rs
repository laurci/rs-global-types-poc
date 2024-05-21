use std::path::PathBuf;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn imports(_input: TokenStream) -> TokenStream {
    let build_script_succeded = if let Some(out_dir) = std::env::var_os("OUT_DIR") {
        PathBuf::from(out_dir).join("adder.rs").exists()
    } else {
        false
    };

    let adder_type_block = if build_script_succeded {
        quote! {
            include!(concat!(env!("OUT_DIR"), "/adder.rs"));
        }
    } else {
        quote! {
            type AdderType = i32;
        }
    };

    quote! {
        #adder_type_block
        type Adder = library::Adder<AdderType>;
    }
    .into()
}

#[proc_macro]
pub fn api_type(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
