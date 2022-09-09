
#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

fn main() {
    let r1 = Rect { 
        w: dbg!(30*2), 
        h:50 };
    println!("r1 = {:?}", r1);
    dbg!(r1);
}
