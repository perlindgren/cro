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
#[derive(Debug)]
pub struct O2 {
    pub i: i32,
}

impl O2 {
    pub fn new(i: i32) -> Self {
        Self { i }
    }

    // sync
    pub fn inc(&mut self) -> i32 {
        println!("o2 inc");
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
    pub fn new(i: i32) -> Self {
        Self {
            state: Resource::new(O2::new(i)),
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

fn main() {
    let o = Cro_O::new(0);
    let o2 = Cro_O2::new(0);

    println!("{}", o.inc());
    println!("{}", o.inc2());
    println!("{}", o2.inc2());

    let mut v: Vec<Box<dyn Runnable>> = vec![];
    v.push(Box::new(o.add(1)));
    v.push(Box::new(o.add(2)));
    v.push(Box::new(o2.add(20)));

    for m in v {
        m.sync();
    }

    println!("o {:?}", o);
}
