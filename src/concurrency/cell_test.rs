// use std::cell::RefCell;
// use std::sync::{Arc, RwLock};
// use std::thread;
// use std::time::Duration;
//
// #[derive(Clone)]
// pub struct AtomicIncrement(Arc<RefCell<usize>>);
//
// impl AtomicIncrement {
//     pub fn new(value: usize) -> Self {
//         AtomicIncrement(Arc::new(RefCell::new(value)))
//     }
//
//     pub fn inc(&self, step: usize) {
//         let s = self.0.clone();
//         let mut s = *s;
//         let i = *s.get_mut();
//         s.replace(i + step);
//     }
//
//     pub fn get(&mut self) -> usize {
//         *self.0.get_mut()
//     }
// }
//
// #[test]
// fn increment() {
//     let mut atomic = AtomicIncrement::new(0);
//
//     let a1 = atomic.clone();
//     thread::spawn(move || {
//         for _ in 0..10 {
//             thread::sleep(Duration::from_millis(20));
//             a1.inc(2);
//         }
//     });
//
//     let a2 = atomic.clone();
//     thread::spawn(move || {
//         for _ in 0..40 {
//             thread::sleep(Duration::from_millis(30));
//             a2.inc(1);
//         }
//     });
//
//     for i in 0..30 {
//         thread::sleep(Duration::from_millis(50));
//         println!("{}: {}", i, atomic.get());
//     }
// }