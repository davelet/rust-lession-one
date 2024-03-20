pub struct ClosureStorage {
    callbacks: Vec<Box<dyn FnMut(i32)>>,
}
impl ClosureStorage {
    pub fn default() -> Self {
        ClosureStorage { callbacks: vec![] }
    }

    pub fn register(&mut self, c: Box<dyn FnMut(i32)>) {
        self.callbacks.push(c)
    }

    pub fn register_generic<FG: FnMut(i32) + 'static>(&mut self, c: FG) {
        // self.callbacks.push(c);
        self.callbacks.push(Box::new(c));
    }

    pub fn call(&mut self, i: i32) {
        self.callbacks.iter_mut().for_each(|c| (*c)(i))
    }
}