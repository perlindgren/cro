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
        let mut o = self.t.lock().unwrap();
        (f)(&mut o)
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
