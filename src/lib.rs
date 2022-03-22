use std::sync::Mutex;

pub trait Runnable {
    fn sync(&self);
}

pub struct Message<'a, T> {
    pub o: &'a Mutex<T>,
    pub f: &'a dyn Fn(&mut T),
}

impl<'a, T> Runnable for Message<'a, T> {
    fn sync(&self) {
        let mut o = self.o.lock().unwrap();
        (self.f)(&mut o);
    }
}

#[test]
fn test_message() {
    let o1 = Mutex::new(0i32);
    let m1 = Message {
        o: &o1,
        f: &|i: &mut i32| *i += 1,
    };

    let o2 = Mutex::new(0i64);
    let m2 = Message {
        o: &o2,
        f: &|i: &mut i64| *i += 10,
    };

    m1.sync();
    m2.sync();

    let mut v: Vec<Box<dyn Runnable>> = vec![];
    v.push(Box::new(m1));
    v.push(Box::new(m2));

    for m in &v {
        m.sync();
    }

    println!("o1 {:?}", o1);
    println!("o2 {:?}", o2);
}
