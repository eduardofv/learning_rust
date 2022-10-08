use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for j in 1..4 {
        let my_tx = tx.clone();
        let handle = thread::spawn(move || {
            for msg_n in 1..10 {
                let msg = String::from(format!("th {j} msg {msg_n}"));
                thread::sleep(Duration::from_millis(100));
                my_tx.send(msg).unwrap()
            }
        });

        println!("{:?}", handle);
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    for received in rx {
        println!("-> {}", received);
    }
}
