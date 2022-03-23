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

    pub fn sync<R>(&self, f: &impl Fn(&mut T) -> R) -> R {
        match self.t.try_lock() {
            Ok(mut o) => (f)(&mut o),
            _ => panic!("deadlock"),
        }
    }
}

pub trait Runnable {
    fn sync(&self);
}

pub struct Message<'a, T> {
    pub o: &'a Resource<T>,
    pub f: Box<dyn Fn(&mut T)>,
}

impl<'a, T> Runnable for Message<'a, T> {
    fn sync(&self) {
        self.o.sync(&self.f)
    }
}
