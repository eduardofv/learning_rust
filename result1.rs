use std::fs::File;

fn main() {
    let file_res = File::open("hello.txt");

    let file = match file_res {
        Ok(file) => file,
        Err(error) => panic!("Problem: {:?}", error),
    };
}
