use std::cell::RefCell;
use std::rc::Rc;

pub struct ClosureStorage {
    callbacks: Vec<Rc<RefCell<dyn FnMut(i32)>>>,
}

impl Clone for ClosureStorage {
    fn clone(&self) -> Self {
        ClosureStorage { callbacks: self.callbacks.clone() }
    }
}

impl ClosureStorage {
    pub fn default() -> Self {
        ClosureStorage { callbacks: vec![] }
    }

    // pub fn register(&mut self, c: Rc<dyn Fn(i32)>) {
    //     self.callbacks.push(c)
    // }

    pub fn register<'a, FG: FnMut(i32) + 'static>(&mut self, c: FG) {
        self.callbacks.push(Rc::new(RefCell::new(c)));
    }

    pub fn call(&mut self, i: i32) {
        self.callbacks.iter().map(|c| c.borrow_mut()).for_each(|mut c| ( c)(i))
    }
}