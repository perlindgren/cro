#![allow(non_camel_case_types)]
use cro::*;

mod o1 {
    use super::*;
    #[derive(Debug)]
    pub struct O {
        pub i: i32,
        pub o2: &'static o2::Cro,
    }

    // later we want to collect in a trait/interface
    pub fn inc() -> Msg<O> {
        // No capture:
        // We don't need to Box here, but it is nice to keep it consistent
        Box::new(|state: &mut O| {
            println!("o1 inc");
            state.i += 1;
            // call o2
            state.o2.call(o2::inc());

            println!("o2 has value {}", state.o2.call(o2::get()));
        })
    }

    pub fn add(v: i32) -> Msg<O> {
        // With capture, Box needed
        Box::new(move |state: &mut O| {
            println!("o1 add v {}", v);
            state.i += v
        })
    }

    pub type Cro = Resource<O>;
}

mod o2 {
    use super::*;
    #[derive(Debug)]
    pub struct O {
        pub i: i32,
    }

    // later we want to collect in a trait/interface
    pub fn inc() -> impl Fn(&mut O) {
        |state: &mut O| {
            println!("o2 inc");
            state.i += 1
        }
    }

    pub fn get() -> impl Fn(&mut O) -> i32 {
        |state: &mut O| {
            println!("o2 get");
            state.i
        }
    }

    // pub fn callback(cb: Message<o1::Cro>) -> Message<o1::Msg<> {
    //     |state: &mut O| {
    //         println!("o2 get");
    //         state.i
    //     }
    // }

    pub fn add(v: i32) -> Msg<O> {
        Box::new(move |state: &mut O| {
            println!("o2 add v {}", v);
            state.i += v
        })
    }

    pub type Cro = Resource<O>;
}

use core::mem::MaybeUninit;

fn main() {
    let mut mu_o1 = Box::new(MaybeUninit::<o1::Cro>::uninit());
    let mut mu_o2 = Box::new(MaybeUninit::<o2::Cro>::uninit());

    let o1: &'static o1::Cro = unsafe { &*mu_o1.as_ptr() };
    let o2: &'static o2::Cro = unsafe { &*mu_o2.as_ptr() };

    mu_o1.write(o1::Cro::new(o1::O { i: 0, o2: o2 }));
    mu_o2.write(o2::Cro::new(o2::O { i: 0 }));

    // no accesses up till this point so should be sound.

    o1.call(o1::inc());
    // o2.call(o2::inc());

    // let mut v: Vec<Box<dyn Runnable>> = vec![];

    // v.push(o1.send(o1::add(3)));
    // v.push(o2.send(o2::add(1)));

    // for m in &v {
    //     m.sync();
    // }

    println!("o1 {:?}", o1);
    println!("o2 {:?}", o2);
}
