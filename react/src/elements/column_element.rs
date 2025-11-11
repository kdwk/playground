use crate::{
    element::FrameExt,
    prelude::{Element, Frame},
};

pub mod prelude {
    pub use super::ColumnElement;
}

pub struct ColumnElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for ColumnElement {
    fn draw(&self) -> Frame {
        self.children
            .iter()
            .map(|child| {
                let mut frame = child.draw();
                frame.align_width();
                frame
            })
            .reduce(|mut acc, mut frame| {
                acc.append(&mut frame);
                acc
            })
            .unwrap_or_else(|| vec!["".to_string()])
    }
}
