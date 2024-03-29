// use std::marker::PhantomData;
use std::{mem, ptr};
// use std::fmt::{Display, Formatter};
//
// pub struct Node<T> {
//     data: T,
//     prev: NodePtr<T>,
//     next: NodePtr<T>,
// }
//
// type NodePtr<T> = *mut Node<T>;
//
// pub struct LinkedList<T> {
//     f1st: NodePtr<T>,
//     last: NodePtr<T>,
//     _marker: PhantomData<T>,
// }
//
pub unsafe fn raw_into_box<T>(raw: *mut T) -> Box<T> {
    mem::transmute(raw)
}

pub fn box_into_raw<T>(b: Box<T>) -> *mut T {
    unsafe { mem::transmute(b) }
}
//
// impl<'a, T> LinkedList<T> {
//     pub fn new() -> Self {
//         LinkedList {
//             f1st: ptr::null_mut(),
//             last: ptr::null_mut(),
//             _marker: PhantomData,
//         }
//     }
//
//     pub fn iter(&self) -> NodeIter<'a, T> {
//         NodeIter::new(self)
//     }
//
//     pub unsafe fn add_last(&mut self, t: T) {
//         let nod = Node {
//             data: t,
//             prev: if self.last.is_null() { ptr::null_mut() } else { self.last },
//             next: ptr::null_mut(),
//         };
//         let new = Box::new(nod);
//         let new = box_into_raw(new);
//         if self.last.is_null() {
//             self.f1st = new;
//         } else {
//             let mut last = *raw_into_box(self.last);
//             last.next = new;
//         }
//         self.last = new;
//     }
// }
//
// impl<T> Display for LinkedList<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let str = "";
//         for nod in self {
//             print!("{}", nod)
//         }
//         write!(f, "{}", str)
//     }
// }
//
// impl<T> Display for Node<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} --> {}", self.data, self.next)
//     }
// }
//
// impl<'a, T> IntoIterator for LinkedList<T> {
//     type Item = Node<T>;
//     type IntoIter = NodeIter<'a, T>;
//
//     fn into_iter(&'a self) -> Self::IntoIter {
//         self.iter()
//     }
// }
//
// pub struct NodeIter<'a, T>
//     where T: 'a
// {
//     list: &'a LinkedList<T>,
//     pointer: NodePtr<T>,
// }
//
// impl<'a, T> NodeIter<'a, T> {
//     fn new(list: &LinkedList<T>) -> Self {
//         NodeIter { list, pointer: list.f1st }
//     }
// }
//
// impl<T> Iterator for NodeIter<'_, T> {
//     type Item = Node<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             if self.pointer.is_null() {
//                 return None;
//             } else {
//                 let c = self.pointer.clone();
//                 unsafe {
//                     let into_box = *raw_into_box(c);
//                     self.pointer = into_box.next;
//                     return Some(into_box);
//                 }
//             }
//         }
//     }
// }
//
// #[test]
// fn test() {
//     let list = LinkedList::new();
//
//     for nod in list {
//         println!("{}", nod)
//     }
// }