use std::collections::btree_map::{Range, Values};

use crate::List::{ Cons, Nil };

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // stores a value in the heap with a pointer to it in the stack
    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
