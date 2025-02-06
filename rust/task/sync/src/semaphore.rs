use std::sync::Arc;
use std::sync::Semaphore;
use std::thread;
use std::vec::Vec;

const NUM_ITERS: u32 = 1000;
const NUM_TASKS: u32 = 100;

fn test_sem_as_mutex() {
    let s = Arc::new(Semaphore::new(1));
    let s2 = s.clone();

    let mut join_handlers = Vec::new();

    for _ in 0..NUM_TASKS {
        let s2 = s2.clone();
        join_handlers.push(thread::spawn(move || {
            for _ in 0..NUM_ITERS {
                let _g = s2.access();
            }
        }));
    }

    drop(s.access());

    for join_handler in join_handlers {
        join_handler.join().unwrap();
    }
}

fn test_sem_as_cvar() {
    let mut join_handlers = Vec::new();

    // Child waits and parent signals
    let s = Arc::new(Semaphore::new(0));

    for _ in 0..NUM_TASKS {
        let s2 = s.clone();
        join_handlers.push(thread::spawn(move || {
            s2.acquire();
        }));
    }

    for _ in 0..NUM_TASKS {
        s.release();
        // Note: on FIFO scheduler, "preempt" is not enabled,
        // just yield manually.
        #[cfg(all(not(feature = "sched_rr"), not(feature = "sched_cfs")))]
        thread::yield_now();
    }

    // Wait for all child tasks to finish.
    while let Some(join_handler) = join_handlers.pop() {
        join_handler.join().unwrap();
    }

    // Parent waits and child signals
    let s = Arc::new(Semaphore::new(0));

    for _ in 0..NUM_TASKS {
        let s2 = s.clone();
        join_handlers.push(thread::spawn(move || {
            s2.release();
        }));
    }

    for _ in 0..NUM_TASKS {
        s.acquire();
    }

    // Wait for all child tasks to finish.
    while let Some(join_handler) = join_handlers.pop() {
        join_handler.join().unwrap();
    }
}

pub fn test_semaphore() {
    println!("Semaphore test...");

    thread::spawn(|| test_sem_as_mutex()).join().unwrap();
    thread::spawn(|| test_sem_as_cvar()).join().unwrap();

    println!("Semaphore test ok");
}
