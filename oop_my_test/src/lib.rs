//ESTA ES UNA PRUEBA SOBRE BORROWS
//  observar como cuando se lee el estado en Post.request_review en el grabbed state
//  el estado Post.state queda en None
//  Tambien como se implementa el debug en el trait

use std::fmt::Debug;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            let grabbed_state = Some(s.request_review());
            println!("Grabbed the state {:?}", grabbed_state);
            println!("internal state is now {:?}", self.state);
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

impl Debug for dyn State {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "State")
    }
}

#[derive(Debug)]
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

#[derive(Debug)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
