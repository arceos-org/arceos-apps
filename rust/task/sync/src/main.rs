#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::thread;

mod barrier;
mod condvar;
mod mutex;
mod rwlock;
mod semaphore;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, synchronization mechanisms test for ArceOS!");

    thread::spawn(|| mutex::test_mutex()).join().unwrap();
    thread::spawn(|| condvar::test_condvar()).join().unwrap();
    thread::spawn(|| barrier::test_barrier()).join().unwrap();
    thread::spawn(|| rwlock::test_rwlock()).join().unwrap();
    thread::spawn(|| semaphore::test_semaphore())
        .join()
        .unwrap();

    println!("All synchronization mechanisms provided by ArceOS seem to work fine, enjoy!");
}
