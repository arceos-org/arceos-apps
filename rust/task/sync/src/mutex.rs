use std::sync::{Arc, Mutex};
use std::thread;

pub fn test_mutex() {
    const NUM_ITERS: u32 = 1000;
    const NUM_TASKS: u32 = 100;
    println!("Mutex test...");

    let m = Arc::new(Mutex::new(0));

    fn inc(m: &Mutex<u32>, val: u32) {
        for _ in 0..NUM_ITERS {
            *m.lock() += val;
        }
    }

    for _ in 0..NUM_TASKS {
        let m2 = m.clone();
        thread::spawn(move || {
            inc(&m2, 1);
        });
        let m2 = m.clone();
        thread::spawn(move || {
            inc(&m2, 2);
        });
    }

    loop {
        let val = m.lock();
        if *val == NUM_ITERS * NUM_TASKS * 3 {
            break;
        }
        drop(val);

        // Note: on FIFO scheduler, "preempt" is not enabled,
        // yield manually to avoid deadlock.
        #[cfg(all(not(feature = "sched_rr"), not(feature = "sched_cfs")))]
        thread::yield_now();
    }
    assert_eq!(*m.lock(), NUM_ITERS * NUM_TASKS * 3);

    println!("Mutex test ok");
}
