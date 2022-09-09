fn main() {
    let v = vec![1, 2, 3];
    let x = v.get(1);
    match x {
        Some(x) => println!("{x}"),
        None => println!("No value"),
    };
}
