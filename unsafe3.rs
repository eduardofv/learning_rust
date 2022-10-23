use std::slice;

fn split(mut text: String, mid: usize) -> (&'static mut [u8], &'static mut [u8]) {
    let mut ptr = text.as_mut_ptr();
    let len = text.len();
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
         )
    }

}

fn main() {
    let s = String::from("A long text");

    let (p1, p2) = split(s, 3);

    println!("{:?}", p1);
    println!("{:?}", p2);
}


//No se bien que esta haciendo
