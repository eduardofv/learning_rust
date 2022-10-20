struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y:0 };// 7 };

    match p {
        Point{ x, y: 0 } => println!("On x at {}. y=0", x),
        Point{ x: 0, y } => println!("On y at {}, x=0", y),
        Point{ x, y } => println!("Somwhere else ({},{})", x, y),
    }
}
