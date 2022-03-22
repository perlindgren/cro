use cro::*;
use std::sync::Mutex;

// user code
#[derive(Debug)]
pub struct State {
    pub i: i32,
}

impl State {
    pub fn new(i: i32) -> Self {
        Self { i }
    }

    // sync
    pub fn inc(&mut self) -> i32 {
        self.i += 1;
        self.i
    }

    // async
    pub fn add(&mut self, v: i32) {
        self.i += v;
    }
}

// auto generated
#[derive(Debug)]
pub struct Cro {
    pub state: Resource<State>,
}

impl Cro {
    pub fn new(i: i32) -> Self {
        Self {
            state: Resource::new(State::new(i)),
        }
    }

    // codegen for sync
    pub fn inc(&self) -> i32 {
        self.state.sync(&State::inc)
    }

    // codegen for async (message)
    pub fn add(&self, v: i32) -> Message<State> {
        Message {
            o: &self.state,
            f: Box::new(move |state| state.add(v)),
        }
    }
}

fn main() {
    let o = Cro::new(0);

    o.inc();

    println!("o {:?}", o);
}
