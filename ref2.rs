fn add_one(val: &mut usize) {
    *val = *val + 1
}

fn add_two(val: &mut usize) {
    *val = *val + 2
}

fn main() {
    let mut x: usize = 10;

    add_one(&mut x);
    add_two(&mut x);

    println!("{x}");
}
