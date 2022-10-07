use std::thread;

fn main() {
    let list = vec![1, 2, 3 ,4];
    println!("{:?}", list);

    thread::spawn(move || println!("thread {:?}", list)).join().unwrap();
    //println!("{:?}", list);
}
