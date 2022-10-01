fn a_fn() -> String {
    let s = String::from("abc");

    s
}

fn main() {
    let mut a_str = a_fn();

    println!("{}", a_str);
    a_str.push('x');
    println!("{}", a_str);
}
