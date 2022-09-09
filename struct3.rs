#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }
}

impl Rect {
    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size, 
        }
    }
}

fn main() {
    let r1 = Rect {w:30, h:50};
    println!("area is {}", r1.area());

    let sq = Rect::square(40);
    println!("area of square is {}", sq.area());
    println!("r1 can hold sq? {}", r1.can_hold(&sq));
}

