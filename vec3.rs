fn main() {
    let mut v = vec![1, 2, 3];

    //let x = &v[0];
    let mut x = &mut v[0];
    //let x = &mut v[0];

    println!("{x}");
    *x = 10;
    //v.push(10);

    //*(&mut v[2]) = 20;

    println!("{x}");
    dbg!(v);
}
