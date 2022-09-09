fn main() {
    let s1 = String::from("hello");

    let len = calc_len(&s1);

    println!("Tamaño de {s1} es {len}");


    let mut s = String::from("Waka");
    change(&mut s);
    println!("s = {s}");


    let mut x: u8 = 1;
    inc(&mut x);
    println!("x es {x}");

}

fn change(some_str: &mut String) {
    some_str.push_str(" waka");
}

fn inc(x: &mut u8) {
    *x = *x + 1;
}

fn calc_len(s: &String) -> usize {
    s.len()
}

