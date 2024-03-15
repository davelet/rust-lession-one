pub trait Minimum {
    fn compare<'a>(&'a self, s:&'a Self) -> &'a Self;
}