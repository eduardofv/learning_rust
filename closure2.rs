fn main() {
    let mut v = vec![1, 2, 3];

    println!("{:?}", v);

    //borrows mutably
    let mut func = || v.push(10);

    //immut borrow
    //println!("{:?}", v);

    func();

    println!("{:?}", v);
}
