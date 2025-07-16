use TraitObjects::{Button, Draw, Screen};
struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        // Logic for gui
    }
}
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 20,
                height: 20,
                options: vec![String::from("Yes"), String::from("No")],
            }),
            Box::new(Button {
                width: 30,
                height: 20,
                leble: String::from("Vishnu"),
            }),
        ],
    };
    screen.run();
}
