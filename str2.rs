fn main() {
    let mut s1 = String::from("Hello ");
    let s2 = String::from("world");

    //let s3 = s1 + &s2;
    s1 += &s2;
    println!("{s1}");
}
