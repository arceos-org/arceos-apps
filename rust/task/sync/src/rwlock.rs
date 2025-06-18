use crate::thread;
use core::sync::atomic::AtomicBool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::vec::Vec;

use rand::{rngs::SmallRng, Rng, SeedableRng};

pub(crate) fn test_rng() -> SmallRng {
    SmallRng::seed_from_u64(0xdead_beef)
}

const NUM_ITERS: u32 = 100;
const NUM_TASKS: u32 = 10;

fn frob() {
    static FINISHED_TASKS: AtomicUsize = AtomicUsize::new(0);

    let r = Arc::new(RwLock::new(()));

    for _ in 0..NUM_TASKS {
        let r = r.clone();
        thread::spawn(move || {
            let mut rng = test_rng();
            for _ in 0..NUM_ITERS {
                if rng.gen_bool(1.0 / (NUM_TASKS as f64)) {
                    drop(r.write());
                } else {
                    drop(r.read());
                }
            }
            FINISHED_TASKS.fetch_add(1, Ordering::SeqCst);
        });
    }

    while FINISHED_TASKS.load(Ordering::Acquire) < NUM_TASKS as usize {
        // Note: on FIFO scheduler, "preempt" is not enabled,
        // yield manually to avoid deadlock.
        #[cfg(all(not(feature = "sched_rr"), not(feature = "sched_cfs")))]
        thread::yield_now();
    }
}

fn test_rw_arc() {
    let arc = Arc::new(RwLock::new(0));
    let arc2 = arc.clone();

    static WRITER_FINISHED: AtomicBool = AtomicBool::new(false);

    thread::spawn(move || {
        let mut lock = arc2.write();
        for _ in 0..NUM_ITERS {
            let tmp = *lock;
            *lock = -1;
            thread::yield_now();
            *lock = tmp + 1;
        }
        WRITER_FINISHED.store(true, Ordering::Release);
    });

    // Readers try to catch the writer in the act
    let mut children = Vec::new();
    for _ in 0..NUM_TASKS {
        let arc3 = arc.clone();
        children.push(thread::spawn(move || {
            let lock = arc3.read();
            assert!(*lock >= 0);
        }));
    }

    // Wait for children to pass their asserts
    for r in children {
        assert!(r.join().is_ok());
    }

    // Wait for writer to finish
    while WRITER_FINISHED.load(Ordering::Acquire) == false {
        // Note: on FIFO scheduler, "preempt" is not enabled,
        // yield manually to avoid deadlock.
        #[cfg(all(not(feature = "sched_rr"), not(feature = "sched_cfs")))]
        thread::yield_now();
    }

    let lock = arc.read();
    assert_eq!(*lock, NUM_ITERS as i32);
}

pub fn test_rwlock() {
    println!("RwLock test...");
    thread::spawn(|| frob()).join().unwrap();
    thread::spawn(|| test_rw_arc()).join().unwrap();
    println!("RwLock test ok");
}
