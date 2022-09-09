fn main() {
    let mut x = 1;

    inc(&mut x);
    println!("{x}");
}

fn inc(pi: &mut i8) {
    *pi += 1;
}
