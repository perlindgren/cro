#![allow(non_camel_case_types)]

use std::sync::Mutex;

#[derive(Debug)]
pub struct Resource<T> {
    t: Mutex<T>,
}

impl<T> Resource<T> {
    pub fn new(t: T) -> Self {
        Self { t: Mutex::new(t) }
    }

    pub fn call<R>(&self, f: impl Fn(&mut T) -> R) -> R {
        match self.t.try_lock() {
            Ok(mut o) => (f)(&mut o),
            _ => panic!("deadlock"),
        }
    }

    pub fn send<'a>(&'a self, f: Box<dyn Fn(&mut T)>) -> Box<Message<'a, T>> {
        Box::new(Message { o: self, f })
    }
}

pub type Msg<T> = Box<dyn Fn(&mut T)>;

pub trait Runnable {
    fn sync(&self);
}

pub struct Message<'a, T> {
    pub o: &'a Resource<T>,
    pub f: Box<dyn Fn(&mut T)>,
}

impl<'a, T> Runnable for Message<'a, T> {
    fn sync(&self) {
        self.o.call(&self.f)
    }
}
