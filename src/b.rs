use macbuild_macros::*;

/// registered functions in submodules are picked up (must be pub)
#[register]
pub fn b() {
    println!("B");
}

