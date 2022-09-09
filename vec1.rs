//Compila y genera error de runtime
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6);
    //v.push(7);
    let third: &i32 = &v[6];

    //dbg!(v);
    
    println!("{third}");
}
