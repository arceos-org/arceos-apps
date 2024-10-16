#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[cfg(feature = "axstd")]
use std::os::arceos::api::config::SMP;
#[cfg(feature = "axstd")]
use std::os::arceos::api::task::{ax_set_current_affinity, AxCpuMask};
#[cfg(feature = "axstd")]
use std::os::arceos::modules::axhal::cpu::this_cpu_id;

const KERNEL_STACK_SIZE: usize = 0x40000; // 256 KiB

const NUM_TASKS: usize = 10;
const NUM_TIMES: usize = 100;
static FINISHED_TASKS: AtomicUsize = AtomicUsize::new(0);

#[allow(clippy::modulo_one)]
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, main task!");
    for i in 0..NUM_TASKS {
        let cpu_id = i % SMP;
        thread::spawn(move || {
            // Initialize cpu affinity here.
            #[cfg(feature = "axstd")]
            assert!(
                ax_set_current_affinity(AxCpuMask::one_shot(cpu_id)).is_ok(),
                "Initialize CPU affinity failed!"
            );

            println!("Hello, task ({})! id = {:?}", i, thread::current().id());
            for _t in 0..NUM_TIMES {
                // Test CPU affinity here.
                #[cfg(feature = "axstd")]
                assert_eq!(this_cpu_id(), cpu_id, "CPU affinity tests failed!");
                thread::yield_now();
            }

            // Change cpu affinity here.
            #[cfg(feature = "axstd")]
            {
                let mut cpumask = AxCpuMask::full();
                cpumask.set(cpu_id, false);
                assert!(
                    ax_set_current_affinity(cpumask).is_ok(),
                    "Change CPU affinity failed!"
                );
            }

            for _t in 0..NUM_TIMES {
                // Test CPU affinity here.
                #[cfg(feature = "axstd")]
                assert_ne!(this_cpu_id(), cpu_id, "CPU affinity changes failed!");
                thread::yield_now();
            }
            let _ = FINISHED_TASKS.fetch_add(1, Ordering::Relaxed);
        });
    }

    while FINISHED_TASKS.load(Ordering::Relaxed) < NUM_TASKS {
        thread::yield_now();
    }

    println!("Task affinity tests run OK!");
}
