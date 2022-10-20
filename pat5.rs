fn main() {
    let x = 2;
    let y = false;

    match y {
        false if x == 2 => println!("Si"),
        true => println!("true"),
    }
    //incomplete matching
}
