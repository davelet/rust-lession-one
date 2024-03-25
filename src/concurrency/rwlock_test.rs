use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct AtomicIncrement(Arc<RwLock<usize>>);

impl AtomicIncrement {
    pub fn new(value: usize) -> Self {
        AtomicIncrement(Arc::new(RwLock::new(value)))
    }

    pub fn inc(&self, step: usize) {
        let lock = self.0.write();
        match lock {
            Ok(mut int) => { *int = *int + step }
            Err(err) => { println!("{}", err) }
        }
    }

    pub fn get(&self) -> usize {
        *self.0.read().unwrap()
    }
}

#[test]
fn increment() {
    let atomic = AtomicIncrement::new(0);

    let a1 = atomic.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(20));
            a1.inc(2);
        }
    });

    let a2 = atomic.clone();
    thread::spawn(move || {
        for _ in 0..40 {
            thread::sleep(Duration::from_millis(30));
            a2.inc( 1);
        }
    });

    for i in 0..30 {
        thread::sleep(Duration::from_millis(50));
        println!("{}: {}", i, atomic.get());
    }
}