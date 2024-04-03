pub mod linkedlist;
use linkedlist::int_linked_list::IntList;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn list() {
        let l = IntList::new();
        assert_eq!(l.size, 1);
    }
}
