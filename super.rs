use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        println!("** {} **", output);
    }
}

impl OutlinePrint for u32 {}

struct Point {
    x: u32,
    y: u32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() {
    let a = 123;

    a.outline_print();

    let p = Point{ x: 10, y: 20 };

    p.outline_print();
}
