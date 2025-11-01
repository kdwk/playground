pub type Frame = Vec<Vec<char>>;

pub trait Element {
    fn draw(&self) -> Frame;
}
