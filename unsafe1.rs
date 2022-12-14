fn main() {

    let mut x: i32 = 5;
    let r_unm = &x as *const i32;
    let r_mut = &mut x as *mut i32;
    let r_mut2 = &mut x as *mut i32;

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

    println!("{x}");
}
