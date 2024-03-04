use crate::definition::SomethingOrNothing::{Nothing, Something};

pub(crate) enum SomethingOrNothing<T> {
    Something(T),
    Nothing
}

impl<T> SomethingOrNothing<T> {

    pub fn new(o: Option<T>) -> Self {
        match o {
            None => {Nothing}
            Some(s) => {Something(s)}
        }
    }

    pub fn to_option(self) -> Option<T> {
        match self {
            Something(t) => {Some(t)}
            Nothing => {None}
        }
    }
}