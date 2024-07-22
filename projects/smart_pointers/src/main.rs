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
