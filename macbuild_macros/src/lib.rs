#![feature(proc_macro)]

#[macro_use] extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;

/// Generates code to import the generated function.
/// 
/// The build script hack exfiltrates the path to the generated file in
/// an environment variable, which we use here to `include!` the file. I
/// tried to do it using `#[path="..."] mod ...;` but that doesn't appear
/// to work with a macro generating the path (cf. RFC issue 1516, issue 48250).
/// 
/// (This could be a `macro_rules!` macro, but I already needed the proc
/// macro crate for `#[register]`, so here we are.)
#[proc_macro]
pub fn macbuild(_input: TokenStream) -> TokenStream {
    let expanded = quote! {
        include!(env!("MACBUILD"));
    };

    expanded.into()
}

/// Handle the `#[register]` attribute.
/// 
/// This has actually been already handled by the build script hack, so
/// the macro itself is a no-op.
#[proc_macro_attribute]
pub fn register(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

