use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn read_user() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();

    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_user_simple() -> Result<String, io::Error> {
    Ok(fs::read_to_string("hello2.txt")?)

}

fn main() {
    let name = read_user().expect("No se pudo leer");
    println!("{name}");

    let name2 = read_user_simple();
    println!("{}", name2.unwrap());
}
