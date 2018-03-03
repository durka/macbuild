#![feature(proc_macro)]

extern crate macbuild_macros;
use macbuild_macros::*;

macbuild!();

fn main() {
    bootstrap();
}

#[register]
pub fn a() {
    println!("A");
}

mod b;

