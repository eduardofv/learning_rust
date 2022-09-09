fn main() {
    let a = [1, 2];
    let b = [1, 2];
    let c = (1, 2);
    let d = (1, 2);

    println!("a==b {}", a==b);
    println!("&a==&b {}", &a==&b);
    println!("c==d {}", c==d);
    println!("&c==&d {}", &c==&d);
}
