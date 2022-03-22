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

    pub fn add(&mut self, v: i32) {
        self.i += v;
    }

    pub fn inc(&mut self) {
        self.i += 1;
    }
}

// auto genereated
#[derive(Debug)]
pub struct Cro {
    pub state: Mutex<State>,
}

impl Cro {
    pub fn new(i: i32) -> Self {
        Self {
            state: Mutex::new(State::new(i)),
        }
    }
    pub fn inc(&self) -> Message<State> {
        Message {
            o: &self.state,
            f: &move |state| state.inc(),
        }
    }

    pub fn add(&self, v: i32) -> Message<State> {
        Message {
            o: &self.state,
            f: &move |state| state.add(v),
        }
    }
}

fn main() {
    let o = Cro::new(0);

    o.inc().sync();

    println!("o {:?}", o);
}
