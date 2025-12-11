use crate::prelude::{DisplayList, Element, Frame, Operation, Size};

pub mod prelude {
    pub use super::CharElement;
}

pub struct CharElement {
    pub c: char,
}

impl Element for CharElement {
    fn draw(&self, _constraint: Size, display_list: &mut DisplayList) {
        display_list.0.push(Operation::PutChar(self.c));
    }
}
