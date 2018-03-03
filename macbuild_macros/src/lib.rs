#![feature(proc_macro)]

#[macro_use] extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn macbuild(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        include!(env!("MACBUILD"));
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn register(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

