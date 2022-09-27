

fn high(name) { fn(name){println!(name)} }

fn main() {
    let f = { |x| x + 1 };

    let f2 = { |y: _| y() + 2 };

    println!("{}", f(10));
    println!("{}", (10));
//    println!("{}", f2(f(10)));
}
