use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        tx.send(String::from("hi2")).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    let received2 = rx.recv().unwrap();
    println!("Got: {}", received2);
}

