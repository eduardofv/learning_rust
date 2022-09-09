fn main() {
/*
    let mut x = 5;
    let y = x;

    println!("x: {x}");
    println!("y: {y}");

    x = 10;
    println!("x: {x}");
    println!("y: {y}");
*/
    let mut s1 = String::from("hello");
    let mut s2 = s1.clone();

    println!("s1: {s1}");
    println!("s2: {s2}");


    s1.push_str(" world");
    println!("s1: {s1}");
    println!("s2: {s2}");

    s2.push_str(" mundo");
    println!("s2: {s2}");

}
