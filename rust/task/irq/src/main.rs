#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[cfg(feature = "axstd")]
use std::os::arceos::modules::axhal;

const NUM_TASKS: usize = 10;
const NUM_TIMES: usize = 100;
static FINISHED_TASKS: AtomicUsize = AtomicUsize::new(0);

fn assert_irq_enabled() {
    #[cfg(feature = "axstd")]
    {
        assert!(
            axhal::arch::irqs_enabled(),
            "Task id = {:?} IRQs should be enabled!",
            thread::current().id()
        );
    }
}

fn assert_irq_disabled() {
    #[cfg(feature = "axstd")]
    {
        assert!(
            !axhal::arch::irqs_enabled(),
            "Task id = {:?} IRQs should be disabled!",
            thread::current().id()
        );
    }
}

fn disable_irqs() {
    #[cfg(feature = "axstd")]
    axhal::arch::disable_irqs()
}

fn enable_irqs() {
    #[cfg(feature = "axstd")]
    axhal::arch::enable_irqs()
}

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    for i in 0..NUM_TASKS {
        thread::spawn(move || {
            println!("Hello, task {}! id = {:?}", i, thread::current().id());

            for _t in 0..NUM_TIMES {
                assert_irq_enabled();
                thread::yield_now();

                disable_irqs();
                assert_irq_disabled();
                enable_irqs();
            }

            println!(
                "Task {}! id = {:?} irq state test ok",
                i,
                thread::current().id()
            );

            let _ = FINISHED_TASKS.fetch_add(1, Ordering::Relaxed);
        });
    }

    println!("Hello, main task, id = {:?}", thread::current().id());
    while FINISHED_TASKS.load(Ordering::Relaxed) < NUM_TASKS {
        assert_irq_enabled();
        thread::yield_now();

        disable_irqs();
        assert_irq_disabled();
        enable_irqs();
    }
    println!("Task irq state tests run OK!");
}
