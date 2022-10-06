fn add_one(val: &mut String) {
    val.push_str(" 1");
}

fn add_two(val: &mut String) {
    val.push_str(" 2");
}

fn main() {
    let mut s = String::from("10");

    add_one(&mut s);
    add_two(&mut s);

    println!("{s}");
}
