- If we have Two Structs we want a method summary in any one who implement the trate 
- Traits are used for shared behaviour 
```rust
use std::fmt::format;

struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}{}", self.author, self.content)
    }
}
struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: String,
    pub retweet: String,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}{}", self.content, self.reply)
    }
}
pub trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let news = NewsArticle {
        author: String::from("vishnu"),
        content: String::from("vishnu"),
        headline: String::from("vishnu"),
    };
    let tweet = Tweet {
        content: String::from("vishnu"),
        reply: String::from("vishnu"),
        retweet: String::from("vishnu"),
        username: String::from("vishnu"),
    };
    println!("Tweet Summary {}", tweet.summarize());
    println!("News Summary {}", news.summarize());
}
```
