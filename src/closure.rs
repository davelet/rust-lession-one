use std::rc::Rc;

pub struct ClosureStorage {
    callbacks: Vec<Rc<dyn Fn(i32)>>,
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

    pub fn register(&mut self, c: Rc<dyn Fn(i32)>) {
        self.callbacks.push(c)
    }

    pub fn register_generic<'a, FG: Fn(i32) + 'static>(&mut self, c: FG) {
        // self.callbacks.push(c);
        self.callbacks.push(Rc::new(c));
    }

    pub fn call(&mut self, i: i32) {
        self.callbacks.iter_mut().for_each(|c| c(i))
    }
}