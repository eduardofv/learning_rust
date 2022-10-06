pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let progress = self.value as f64 / self.max as f64;

        if progress >= 1.0 {
            self.messenger.send("Over quota");
        } else if progress >= 0.9 {
            self.messenger.send("At 90% of quota");
        } else if progress >= 0.75 {
            self.messenger.send("At 75% of quota");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
            //self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn message_at_75p() {
        let mock_msg = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_msg, 100);

        tracker.set_value(80);

        assert_eq!(mock_msg.sent_messages.borrow().len(), 1);
    }
}

