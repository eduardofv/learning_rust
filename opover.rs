use std::ops::Add;

#[derive(Debug)]
struct Millimiters(u32);

#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimiters {
    type Output = Millimiters;

    fn add(self, other:Meters) -> Millimiters {
        Millimiters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let a = Millimiters(1000);
    let b = Meters(1);

    let c = a + b;
    println!("{:?}", c);
}
