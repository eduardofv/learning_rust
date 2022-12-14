#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Box<List>),
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);
}
