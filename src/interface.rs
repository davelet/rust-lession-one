pub trait Minimum : Copy {
    fn compare(&self, s: Self) -> Self;
}