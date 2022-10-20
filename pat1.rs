fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Es 50"),
        Some(y)  => println!("y = {y}"),
        _ => println!("default x: {:?}", x),
    }

    println!("x al final {:?}, y = {y}", x);
}
