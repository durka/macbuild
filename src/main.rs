#![feature(proc_macro)]

// import macbuild!() and #[register]
extern crate macbuild_macros;
use macbuild_macros::*;

// this generates code to import bootstrap()
macbuild!();

fn main() {
    bootstrap(); // this calls all functions annotated with #[register]
}

/// first registered function
#[register]
pub fn a() {
    println!("A");
}

/// this module contains the second registered function
mod b;

