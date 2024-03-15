use std::fmt::Display;
use crate::definition::SomethingOrNothing::{Nothing, Something};

pub(crate) enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}

impl<T: Display> SomethingOrNothing<T> {
    pub fn new(o: Option<T>) -> Self {
        match o {
            None => { Nothing }
            Some(s) => { Something(s) }
        }
    }

    pub fn to_option(self) -> Option<T> {
        match self {
            Something(t) => { Some(t) }
            Nothing => { None }
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Something(t) => { t }
            Nothing => { panic!("no value") }
        }
    }

    pub fn print(&self) {
        match self {
            Something(n) => {println!("{} is least", n)}
            Nothing => { println!("none") }
        }
    }
}

// impl<T: Clone> Clone for SomethingOrNothing<T> {
//     fn clone(&self) -> Self {
//         match *self {
//             Something(ref v) => { Something(v.clone()) }
//             Nothing => { Nothing }
//         }
//     }
// }