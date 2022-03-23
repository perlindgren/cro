use cro_derive::*;

#[cro_state]
// #[derive(Debug)]
struct O {
    pub i: i32,
    pub o2: &'static O2,
}

#[cro_impl]
impl O {
    pub fn new(i: i32, o2: &'static O2) -> Self {
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
        let d = self.o2.dec();
        println!("o add");
        self.i += v + d;
    }
}

fn main() {
    println!("here");
}
