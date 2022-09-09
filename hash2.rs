use std::collections::HashMap;

fn main() {
    let mut h = HashMap::new();
    let mut s = String::from("hola");

    h.insert(1, &s);
    //h.insert(2, &s);

    println!("{:?}", h);

    let s2 = String::from("hola2");
    h.insert(1, &s2);

    println!("{:?}", h);

    s.push_str(" adios");

    println!("{:?}", s);
}
