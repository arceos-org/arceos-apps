#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println;

#[cfg(feature = "axstd")]
extern crate axhal_plat_impl;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, world!");
}
