
use std::fmt;
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest is {}", self.x);
        } else {
            println!("Largest is {}", self.y);
        }
    }
}

impl<T: Display> Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p1 = Pair::new(1, 2);
    p1.cmp_display();
    println!("{p1}");
    
    let p2 = Pair::new(5, 4);
    p2.cmp_display();
    println!("{p2}");

    let pp = Pair::new(p1, p2);
    //pp.cmp_display();
    println!("{pp}");

    let pp_str = pp.to_string();
    println!("{}", pp_str);
}
