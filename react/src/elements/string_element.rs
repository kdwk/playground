use crate::prelude::{Element, Frame, FrameExt};

pub mod prelude {
    pub use super::StringElement;
}

pub struct StringElement {
    pub s: String,
}

impl Element for StringElement {
    fn draw(&self) -> Frame {
        vec![self.s.clone()]
    }
}
