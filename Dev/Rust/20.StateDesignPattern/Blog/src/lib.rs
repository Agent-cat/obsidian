pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    // It is a construstor
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::from(""),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        ""
    }
}
pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}
struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
