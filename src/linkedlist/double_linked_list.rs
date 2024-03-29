use std::marker::PhantomData;
use std::{mem, ptr};
use std::fmt::{Display, Formatter};

pub struct Node<T> {
    data: T,
    prev: NodePtr<T>,
    next: NodePtr<T>,
}

type NodePtr<T> = *mut Node<T>;

pub struct LinkedList<T> {
    f1st: NodePtr<T>,
    last: NodePtr<T>,
    _marker: PhantomData<T>,
}

pub unsafe fn raw_into_box<T>(raw: *mut T) -> Box<T> {
    mem::transmute(raw)
}

pub fn box_into_raw<T>(b: Box<T>) -> *mut T {
    unsafe { mem::transmute(b) }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            f1st: ptr::null_mut(),
            last: ptr::null_mut(),
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> NodeIter<T> {
        NodeIter::new(self.f1st)
    }

    pub unsafe fn add_last(&mut self, t: T) {
        let nod = Node {
            data: t,
            prev: self.last,
            next: ptr::null_mut(),
        };
        let new = Box::new(nod);
        let new = box_into_raw(new);
        if self.last.is_null() {
            self.f1st = new;
        } else {
            let mut last = &mut *self.last;
            last.next = new;
        }
        self.last = new;
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = Node<T>;
    type IntoIter = NodeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct NodeIter<T> {
    pointer: NodePtr<T>,
}

impl<T> NodeIter<T> {
    fn new(pointer: NodePtr<T>) -> Self {
        NodeIter { pointer }
    }
}

impl<T> Iterator for NodeIter<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.pointer.is_null() {
                return None;
            } else {
                let c = self.pointer.clone();
                unsafe {
                    let into_box = *raw_into_box(c);
                    self.pointer = into_box.next;
                    return Some(into_box);
                }
            }
        }
    }
}

#[test]
fn test() {
    let mut list = LinkedList::new();
    unsafe { list.add_last(1); }
    unsafe { list.add_last(20); }
    unsafe { list.add_last(520); }

    for ll in list {
        unsafe { println!("{:?}", ll.data); }
    }

}