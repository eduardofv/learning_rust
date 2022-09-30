fn main() {
    let v1 = vec![1, 2, 3];

    let mapped = v1.iter().map(|x| x + 1);

    println!("{:?}", mapped);

    let res: Vec<_> = mapped.collect();
    println!("{:?}", res);
}
