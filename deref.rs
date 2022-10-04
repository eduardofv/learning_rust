use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(val: T) -> MyBox<T>{
        MyBox(val)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {name}");
}

fn main() {
    let x = 10;
    let y = MyBox::new(x);

    println!("{x}");
    println!("{}", *y);
    println!("{}", *y);

    let name = MyBox::new(String::from("Rust"));
    hello(&name);
}
