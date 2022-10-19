
#[derive(Debug)]
pub struct Post {
    content: String, 
}

#[derive(Debug)]
pub struct DraftPost {
    content: String,
}

#[derive(Debug)]
pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();
    println!("{:?}", post);

    post.add_text("Bla");
    println!("{:?}", post);

    let post = post.request_review();
    println!("{:?}", post);

    let post = post.approve();
    println!("{:?}", post);

    //sería bueno tener alguna forma de evitar esto en compilacion
    //si tanto enfasis se hace en evitar esos errores (en el book
    //habla de evitar el que se use content en post no aprobados)
    let invalid_post = PendingReviewPost { content: String::from("123"), };
    println!("{:?}", invalid_post);
}
