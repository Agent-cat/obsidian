pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    // If we use a normal vector it stores only one type of data
    pub components: Vec<Box<dyn Draw>>, // dyn stands for dynamic dispatch
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub leble: String,
}
impl Draw for Button {
    fn draw(&self) {
        //Gui Logic
    }
}
