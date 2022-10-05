#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }
    

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    //println!("c: {:?}", c);
    println!("{}", Rc::strong_count(&a));
}
