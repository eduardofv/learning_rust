fn main() {
    let b1 = Box::new(5);
    println!("{b1}");

    let s1 = Box::new(String::from("hola"));
    println!("{s1}");

    let v1 = Box::new(vec![1, 2, 3]);
    println!("{:?}", v1);
}
