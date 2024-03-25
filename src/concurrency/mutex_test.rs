use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct AtomicIncrement(Arc<Mutex<usize>>);

impl AtomicIncrement {
    pub fn new(value: usize) -> Self {
        AtomicIncrement(Arc::new(Mutex::new(value)))
    }

    pub fn inc(&self, step: usize) {
        let lock = self.0.lock();
        match lock {
            Ok(mut int) => { *int = *int + step }
            Err(err) => { println!("{}", err) }
        }
    }

    fn cas(&self, test: usize, offset: usize) -> bool {
        let lock = self.0.lock();
        match lock {
            Ok(mut mutex) => {
                if *mutex == test {
                    *mutex = *mutex + offset;
                    return true;
                }
                println!("cas:{} -- {}", test, *mutex);
            }
            Err(err) => { println!("{}", err) }
        }
        return false;
    }

    pub fn get(&self) -> usize {
        *self.0.lock().unwrap()
        // *self.0.lock().unwrap_or_else(|e| { e.into_inner() })
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
            let i1 = a2.get();
            thread::sleep(Duration::from_millis(30));
            a2.cas(i1, 1);
        }
    });

    for i in 0..30 {
        thread::sleep(Duration::from_millis(50));
        println!("{}: {}", i, atomic.get());
    }
}