use mocks::*;

struct ConsoleMessenger {}

impl ConsoleMessenger {
    fn new() -> ConsoleMessenger {
        ConsoleMessenger {}
    }
}

impl Messenger for ConsoleMessenger {
    fn send(&self, message: &str) {
        println!("{}", message);
    }
}
    
fn main() {
    let my_messenger = ConsoleMessenger::new();
    let mut my_tracker = LimitTracker::new(&my_messenger, 100);

    my_tracker.set_value(80);
}
