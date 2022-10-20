fn main() {
    let s = Some(String::from("hola"));

    //if let Some(_s) = s { //this moves the value
    if let Some(_) = s { //this doesn't move the value
        println!("a string");
    }
    
    println!("{:?}", s);
}
