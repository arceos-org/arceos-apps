use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::vec::Vec;

const NUM_TASKS: u32 = 10;
const NUM_ITERS: u32 = 100;

fn test_barrier_rendezvous() {
    static BARRIER: Barrier = Barrier::new(NUM_TASKS as usize);
    static FINISHED_TASKS: AtomicUsize = AtomicUsize::new(0);

    fn rendezvous() {
        for _ in 0..NUM_ITERS {
            BARRIER.wait();
        }
        FINISHED_TASKS.fetch_add(1, Ordering::SeqCst);
    }

    for _ in 0..NUM_TASKS {
        thread::spawn(rendezvous);
    }

    // Wait for all threads to finish.
    while FINISHED_TASKS.load(Ordering::SeqCst) < NUM_TASKS as usize {
        // Note: on FIFO scheduler, "preempt" is not enabled,
        // yield manually to avoid deadlock.
        #[cfg(all(not(feature = "sched_rr"), not(feature = "sched_cfs")))]
        thread::yield_now();
    }
}

fn test_wait_result() {
    static LEADER_FOUND: AtomicBool = AtomicBool::new(false);
    static FINISHED_TASKS: AtomicUsize = AtomicUsize::new(0);

    let barrier = Arc::new(Barrier::new(NUM_TASKS as _));

    let mut join_handlers = Vec::new();

    for _ in 0..NUM_TASKS - 1 {
        let c = barrier.clone();
        join_handlers.push(thread::spawn(move || {
            let is_leader = c.wait().is_leader();
            if is_leader {
                LEADER_FOUND.store(true, Ordering::SeqCst);
            }
            FINISHED_TASKS.fetch_add(1, Ordering::SeqCst);
        }));
    }

    // At this point, all spawned threads should be blocked,
    // so we shouldn't get a true value from `LEADER_FOUND`.
    assert!(!LEADER_FOUND.load(Ordering::Acquire));

    let leader_found = barrier.wait().is_leader();
    if leader_found {
        LEADER_FOUND.store(true, Ordering::SeqCst);
    }

    // Wait for all threads to finish.
    for join_handler in join_handlers {
        join_handler.join().unwrap();
    }

    assert_eq!(
        FINISHED_TASKS.load(Ordering::Relaxed),
        NUM_TASKS as usize - 1
    );
    // Now, the barrier is cleared and we should get true from `LEADER_FOUND`.
    assert!(LEADER_FOUND.load(Ordering::Relaxed));
}

pub fn test_barrier() {
    println!("Barrier test...");
    thread::spawn(|| test_barrier_rendezvous()).join().unwrap();
    thread::spawn(|| test_wait_result()).join().unwrap();
    println!("Barrier test OK");
}
