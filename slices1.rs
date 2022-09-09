
fn main() {
    let mut text = String::from("this is a string");

    let first = first_w(&text);
    //text.clear();
    println!("first: {first}");
}


fn first_w(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
