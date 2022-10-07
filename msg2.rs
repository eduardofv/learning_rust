use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(100));
            let received = match rx.try_recv() {
                Ok(msg) => println!("recibiendo {:?}", msg),
                Err(_) => println!("."),
            };
        }
    });

    tx.send(String::from("hola")).unwrap();
    thread::sleep(Duration::from_millis(500));
    tx.send(String::from("mundo")).unwrap();

    thread::sleep(Duration::from_millis(1000));
}

