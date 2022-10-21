fn main() {

    let mut x = String::from("1223");
    let r_unm = &x as *const String;
    let r_mut = &mut x as *mut String;
    let r_mut2 = &mut x as *mut String;

    //no permitido, una ref mutable y una inmutable
    //println!("{}", *r_unm);
    unsafe {
        println!("{}", *r_unm);
        println!("{}", *r_mut);
        println!("{}", *r_mut2);
    }

    println!("{x}");
    drop(x);

    unsafe {
        println!("{}", *r_unm);
        println!("{}", *r_mut);
        println!("{}", *r_mut2);
    }

    //already dropped, can't be used
    //println!("{x}");
}
