fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    let val1 = v1_iter.next();
    println!("{}", val1.unwrap());

    println!("v1: {:?}", v1);

    for val2 in v1_iter() {
        println!("{}", val2);
    }
}
