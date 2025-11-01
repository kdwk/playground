use crate::elements::element::{Element, Frame};

pub struct NumberElement {
    pub i: i32,
}

impl Element for NumberElement {
    fn draw(&self) -> Frame {
        vec![self.i.to_string().chars().collect()]
    }
}
