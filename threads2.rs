use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = vec![];

    for j in 1..3 {
        let handle = thread::spawn(move || {
            for i in 1..10 {
                println!("In Subthread {} count {}", j, i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }

    for i in 1..5 {
        println!("In main thread count {i}");
        thread::sleep(Duration::from_millis(1));
    }

    for h in handles {
        h.join().unwrap();
    }
}
