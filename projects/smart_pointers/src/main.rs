use std::collections::btree_map::{ Range, Values };
use std::ops::Deref;
use crate::List::{ Cons, Nil };

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

// custom defined smart pointer like Box<T>
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// add Dereferencing method to custom Box
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
}

pub fn set_value(&mut self, value:usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
        self.messenger.send("you are over quota");
    } else if percentage_of_max >= 0.9 {
        self.messenger.send("you are on quota");
    }else if percentage_of_max >= 0.75 {
        self.messenger.send("you are 3/4 on you quota");
    }
}






fn main() {
    /*
    // stores a value in the heap with a pointer to it in the stack
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    */
    // using custom smart pointer with its custom deref trait
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
