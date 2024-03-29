use std::fmt::{Debug, Display, Formatter, Pointer};
use std::ptr;
use crate::linkedlist::double_linked_list::{box_into_raw, raw_into_box};

struct IntList {
    head: *mut Node,
    tail: *mut Node,
    size: usize,
}

#[derive(Copy)]
#[derive(Clone)]
struct Node {
    data: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({:?} {:?})", self.data, self.prev, self.next)
    }
}

impl IntList {
    fn new() -> Self {
        IntList {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            size: 0,
        }
    }

    unsafe fn add_last(&mut self, v: i32) {
        let node = Node { data: v, prev: self.tail, next: ptr::null_mut() };
        let raw = box_into_raw(Box::new(node));
        let node = *raw;
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
        println!("added:{}", node);
    }

    fn out_put(&self) {
        match self.size {
            0 => { println!("empty list") }
            _ => unsafe {
                let mut cur = self.head;
                loop {
                    if cur.is_null() { break; }
                    let n = *(cur);
                    print!("{} -> ", n.to_string());
                    cur = n.next;
                }
                print!("(end)")
            }
        }
        println!()
    }
}

impl IntoIterator for IntList {
    type Item = i32;
    type IntoIter = MyIter;

    fn into_iter(self) -> Self::IntoIter {
        MyIter::new(self.head)
    }
}

struct MyIter {
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


#[test]
fn test() {
    let mut list = IntList::new();
    unsafe { list.add_last(3); }
    unsafe { list.add_last(45); }
    unsafe { list.add_last(900); }
    unsafe { list.add_last(200); }
    println!("{}", list.size);
    list.out_put();
    println!("{:?}", list.head);
    println!("{:?}", list.tail);
    // unsafe { println!("{:?}", raw_into_box(list.head).next); }
    for i in list {
        println!("{}", i)
    }
    // println!("{}", list.size);
}