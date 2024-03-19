pub trait Minimum {
    fn compare<'a>(&'a self, s:&'a Self) -> &'a Self;
}

pub trait Action {
    fn do_action(&mut self, d: u8);
}