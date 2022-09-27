fn main() {
    let mut s = String::from("hola");

    let s1 = &s;
    let s2 = &s;

    //println!("{} {}", s1, s2);

    let s3 = &mut s;

    //println!("{} {}", s1, s3);
    println!("{}", s3);

}
