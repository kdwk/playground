use crate::prelude::{Element, Frame};

pub mod prelude {
    pub use super::CharElement;
}

pub struct CharElement {
    pub c: char,
}

impl Element for CharElement {
    fn draw(&self) -> Frame {
        vec![self.c.to_string()]
    }
}
