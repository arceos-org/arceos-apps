#![no_std]
#![no_main]

extern crate axstd as std;

use std::os::arceos::modules::axnet;

#[no_mangle]
fn main() {
    axstd::println!("Benchmarking bandwidth...");
    axnet::bench_transmit();
    // axnet::bench_receive();
}
