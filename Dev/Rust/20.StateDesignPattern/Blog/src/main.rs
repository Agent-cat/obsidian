use Blog::Post;
fn main() {
    // Not completed finis it from video 33
    let mut post = Post::new();
    post.add_text("My name is vishnu");
    assert_eq!("", post.content());
    //  post.request_review();
    assert_eq!("", post.content());
    // post.approve();
    assert_eq!("My name is vishnu", post.content());
}
