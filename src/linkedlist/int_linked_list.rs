use std::fmt::{Debug, Display, Formatter, Pointer};
use std::ptr;
use criterion::black_box;
use crate::linkedlist::double_linked_list::{box_into_raw, raw_into_box};

#[derive(Copy, Clone)]
pub struct IntList {
    head: *mut Node,
    tail: *mut Node,
    pub size: usize,
}

struct Node {
    data: i32,
    prev: *mut Node,
    next: *mut Node,
}

#[test]
fn test() {
    unsafe {
        let mut list = IntList::from_iter((0..10).map(black_box));
        // b.iter(|| {
        //     let list = list.clone();
        for i in list {
            println!("{}", i)
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({:?} {:?})", self.data, self.prev, self.next)
    }
}

impl IntList {
    pub fn new() -> Self {
        IntList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            size: 0,
        }
    }

    pub unsafe fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let mut l = Self::new();
        for i in iter {
            l.add_last(i);
        }
        l
    }

    pub unsafe fn add_last(&mut self, v: i32) {
        let node = Node { data: v, prev: self.tail, next: ptr::null_mut() };
        let raw = box_into_raw(Box::new(node));
        match self.size {
            0 => {
                self.head = raw;
                self.tail = raw;
                // println!("adding first:{}", node);
            }
            _ => unsafe {
                let mut last = &mut *self.tail;
                last.next = raw;
                // (*self.tail).next = raw;
                self.tail = raw;
            }
        }
        self.size += 1;
        // println!("added:{}", node);
    }

    // pub fn out_put(&self) {
    //     match self.size {
    //         0 => { println!("empty list") }
    //         _ => unsafe {
    //             let mut cur = self.head;
    //             loop {
    //                 if cur.is_null() { break; }
    //                 let n = *(cur);
    //                 print!("{} -> ", n.to_string());
    //                 cur = n.next;
    //             }
    //             print!("(end)")
    //         }
    //     }
    //     println!()
    // }
}

impl IntoIterator for IntList {
    type Item = i32;
    type IntoIter = MyIter;

    fn into_iter(self) -> Self::IntoIter {
        MyIter::new(self.head)
    }
}

pub struct MyIter {
    index: *mut Node,
}

impl MyIter {
    fn new(head: *mut Node) -> Self {
        MyIter {
            index: head,
        }
    }
}

impl Iterator for MyIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.index.is_null() {
                None
            } else {
                let b = *Box::from_raw(self.index);
                self.index = b.next;
                Some(b.data)
            }
        }
    }
}
