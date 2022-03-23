#![allow(non_camel_case_types)]
use cro::*;

// user code
#[derive(Debug, Copy, Clone)]
pub struct O {
    pub i: i32,
    pub o2: &'static Cro_O2,
}

impl O {
    pub fn new(i: i32, o2: &'static Cro_O2) -> Self {
        Self { i, o2 }
    }

    // sync
    pub fn inc(&mut self) -> i32 {
        println!("o inc");
        self.i += 1;
        self.i
    }

    // sync
    pub fn inc2(&mut self) -> i32 {
        println!("o inc2");
        self.inc()
    }

    // async
    pub fn add(&mut self, v: i32) {
        let v = self.o2.inc();
        println!("o add");
        self.i += v;
    }
}

// auto generated
#[derive(Debug)]
pub struct Cro_O {
    pub state: Resource<O>,
}

impl Cro_O {
    pub fn new(i: i32, o2: &'static Cro_O2) -> Self {
        Self {
            state: Resource::new(O::new(i, o2)),
        }
    }

    // codegen for sync
    pub fn inc(&self) -> i32 {
        self.state.sync(&O::inc)
    }

    // codegen for sync
    pub fn inc2(&self) -> i32 {
        self.state.sync(&O::inc2)
    }

    // codegen for async (message)
    pub fn add(&self, v: i32) -> Message<O> {
        // should we automatically enqueue the message?
        Message {
            o: &self.state,
            f: Box::new(move |state| state.add(v)),
        }
    }
}

// user code

#[derive(Debug, Copy, Clone)]
pub struct O2 {
    pub i: i32,
    pub o1: &'static Cro_O,
}

impl O2 {
    pub fn new(i: i32, o1: &'static Cro_O) -> Self {
        Self { i, o1 }
    }

    // sync
    pub fn inc(&mut self) -> i32 {
        println!("o2 inc");
        (self.o1).inc();
        self.i += 1;
        self.i
    }

    // sync
    pub fn inc2(&mut self) -> i32 {
        println!("o2 inc2");
        self.inc()
    }

    // async
    pub fn add(&mut self, v: i32) {
        println!("o2 add");
        self.i += v;
    }
}

// auto generated
#[derive(Debug)]
pub struct Cro_O2 {
    pub state: Resource<O2>,
}

impl Cro_O2 {
    pub fn new(i: i32, o1: &'static Cro_O) -> Self {
        Self {
            state: Resource::new(O2::new(i, o1)),
        }
    }

    // codegen for sync
    pub fn inc(&self) -> i32 {
        self.state.sync(&O2::inc)
    }

    // codegen for sync
    pub fn inc2(&self) -> i32 {
        self.state.sync(&O2::inc2)
    }

    // codegen for async (message)
    pub fn add(&self, v: i32) -> Message<O2> {
        // should we automatically enqueue the message?
        Message {
            o: &self.state,
            f: Box::new(move |state| state.add(v)),
        }
    }
}

use core::mem::MaybeUninit;

fn main() {
    // needs some convenience wrapper
    let mut mu_o1 = Box::new(MaybeUninit::<Cro_O>::uninit());
    let mut mu_o2 = Box::new(MaybeUninit::<Cro_O2>::uninit());

    let p_mu_o1: &'static Cro_O = unsafe { &*mu_o1.as_ptr() };
    let p_mu_o2: &'static Cro_O2 = unsafe { &*mu_o2.as_ptr() };

    mu_o1.write(Cro_O::new(0, p_mu_o2));
    mu_o2.write(Cro_O2::new(0, p_mu_o1));

    let o1 = unsafe { mu_o1.assume_init() };
    let o2 = unsafe { mu_o2.assume_init() };

    println!("{}", o2.inc());

    let m = o1.add(2);

    println!("message created");
    println!("o1 {:?}", o1);
    println!("o2 {:?}", o2);

    println!("sync call");
    m.sync();
    println!("sync return");

    println!("o1 {:?}", o1);
    println!("o2 {:?}", o2);
}
