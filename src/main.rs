#![allow(non_camel_case_types)]
use cro::*;

// user code
#[derive(Debug)]
pub struct O {
    pub i: i32,
}

impl O {
    pub fn new(i: i32) -> Self {
        Self { i }
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
    pub fn new(i: i32) -> Self {
        Self {
            state: Resource::new(O::new(i)),
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

pub struct O2<'a, O_1, O_2> {
    pub i: i32,
    pub o_inc: (&'a O_1, fn(&'a O_1) -> i32),
    pub o_add: (&'a O_2, fn(&'a O_2, i32) -> Message<O_2>),
}

impl<'a, O_1, O_2> O2<'a, O_1, O_2> {
    pub fn new(
        i: i32,
        o_inc: (&'a O_1, fn(&'a O_1) -> i32),
        o_add: (&'a O_2, fn(&'a O_2, i32) -> Message<O_2>),
    ) -> Self {
        Self { i, o_inc, o_add }
    }

    // sync
    pub fn inc(&mut self) -> i32 {
        println!("o2 inc");
        self.o_inc.1(self.o_inc.0);
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
pub struct Cro_O2<'a, O_1, O_2> {
    pub state: Resource<O2<'a, O_1, O_2>>,
}

impl<'a, O_1, O_2> Cro_O2<'a, O_1, O_2> {
    pub fn new(
        i: i32,
        o_inc: (&'a O_1, fn(&'a O_1) -> i32),
        o_add: (&'a O_2, fn(&'a O_2, i: i32) -> Message<O_2>),
    ) -> Self {
        Self {
            state: Resource::new(O2::new(i, o_inc, o_add)),
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
    pub fn add(&'a self, v: i32) -> Message<O2<O_1, O_2>> {
        // should we automatically enqueue the message?
        Message {
            o: &self.state,
            f: Box::new(move |state| state.add(v)),
        }
    }
}

fn main() {
    let o = Cro_O::new(0);
    let o2 = Cro_O2::new(0, (&o, Cro_O::inc), (&o, Cro_O::add));

    // println!("{}", o2.inc());
    // println!("{}", o.inc());
    // println!("{}", o.inc2());
    // println!("{}", o2.inc2());

    // let mut v: Vec<Box<dyn Runnable>> = vec![];
    // v.push(Box::new(o.add(1)));
    // v.push(Box::new(o.add(2)));
    // v.push(Box::new(o2.add(20)));

    // for m in v {
    //     m.sync();
    // }

    println!("o {:?}", o);
}
