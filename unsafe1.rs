fn main() {

    let mut x: i32 = 5;
    let r_unm = &x as *const i32;
    let r_mut = &mut x as *mut i32;

    unsafe {
        println!("{}", *r_unm);
        println!("{}", *r_mut);
    }
}
