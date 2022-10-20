fn foo(x: String, _: String) {
    println!("moving {x} ");
}

fn main() {
    let x = String::from("a");
    let y = String::from("b");

    println!("{x} {y}");
    foo(x, y);
    //println!("{x}"); x was moved into fn
    println!("{y}"); //y was also moved despite not used
}

