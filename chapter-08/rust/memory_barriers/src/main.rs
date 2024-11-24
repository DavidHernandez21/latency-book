use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let memory_violation = Arc::new(AtomicBool::new(false));
    (0..1_000_000).for_each(|_| {
        let value = Arc::new(AtomicUsize::new(0));
        let done = Arc::new(AtomicBool::new(false));
        let value1 = value.clone();
        let done1 = done.clone();
        let inner_memory_violation = memory_violation.clone();
        let t1 = thread::spawn(move || {
            value1.store(1, Ordering::Relaxed);
            // ordring release ensures that the store to done1 happens after the store to value1
            done1.store(true, Ordering::Release);
        });
        let t2 = thread::spawn(move || {
            // ordring acquire ensures that the load from done1 happens after the store to done1
            let done = done.load(Ordering::Acquire);
            let value = value.load(Ordering::Relaxed);
            
            match (value, done) {
                (1, false) | (0, true) => {
                    inner_memory_violation.store(true, Ordering::Relaxed);
                    println!("value = {value}, done = {done}");}
                _ => (),
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();

    });

    
    if memory_violation.load(Ordering::Relaxed)   {
        println!("Memory violation detected");
        return;
    }
    println!("Memory violation not detected");
}
