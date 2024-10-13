#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use cpumask::CpuMask;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature = "axstd")]
use axtask::TaskInner;
#[cfg(feature = "axstd")]
use std::os::arceos::modules::axconfig;
#[cfg(feature = "axstd")]
use std::os::arceos::modules::axhal;
#[cfg(feature = "axstd")]
use std::os::arceos::modules::axtask;

const KERNEL_STACK_SIZE: usize = 0x40000; // 256 KiB

const NUM_TASKS: usize = 10;
const NUM_TIMES: usize = 100;
static FINISHED_TASKS: AtomicUsize = AtomicUsize::new(0);

#[allow(clippy::modulo_one)]
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Hello, main task!");
    for i in 0..NUM_TASKS {
        let cpu_id = i % axconfig::SMP;
        let task = TaskInner::new(
            move || {
                println!("Hello, task ({})! id = {:?}", i, axtask::current().id());
                for _t in 0..NUM_TIMES {
                    // Test CPU affinity here.
                    assert_eq!(axhal::cpu::this_cpu_id(), cpu_id, "CPU affinity failed!");
                    axtask::yield_now();
                }

                // Change cpu affinity here.
                let mut cpumask = CpuMask::full();
                cpumask.set(cpu_id, false);
                assert!(
                    axtask::set_current_affinity(cpumask),
                    "Change CPU affinity failed!"
                );

                for _t in 0..NUM_TIMES {
                    // Test CPU affinity here.
                    assert_ne!(axhal::cpu::this_cpu_id(), cpu_id, "CPU affinity failed!");
                    axtask::yield_now();
                }
                let _ = FINISHED_TASKS.fetch_add(1, Ordering::Relaxed);
            },
            "".into(),
            crate::KERNEL_STACK_SIZE,
        );

        // Initialized cpu affinity here.
        task.set_cpumask(CpuMask::one_shot(cpu_id));
        axtask::spawn_task(task);
    }

    while FINISHED_TASKS.load(Ordering::Relaxed) < NUM_TASKS {
        axtask::yield_now();
    }

    println!("Task affinity tests run OK!");
}
