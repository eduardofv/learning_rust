fn main() {
    let mut s1 = String::from("Hola");

    s1 = takes_own(s1);
    println!("{s1}");


    let x = 5;
    makes_copy(x);
    println!("{x}");

    let y = 10;
    println!("{}", y);
    takes_own_int(y);
    println!("{}", y);

    let s2 = String::from("Otro");
    println!("{}", s2);
    takes_own(s2);
    //println!("{}", s2);
}

fn takes_own(some_str: String) -> String {
    println!("  in fn: {some_str}");
    some_str
}

fn takes_own_int(some_int: i32) {
    println!("{}", some_int);
}

fn makes_copy(mut some_int: i32) {
    println!("  in fn: {some_int}");
    some_int += 1;
}
