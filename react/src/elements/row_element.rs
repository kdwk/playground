use crate::prelude::{Element, Frame, FrameExt, ensure_same_height};

pub mod prelude {
    pub use super::RowElement;
}

pub struct RowElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for RowElement {
    fn draw(&self) -> Frame {
        self.children
            .iter()
            .map(|child| {
                let mut frame = child.draw();
                frame.align_width();
                frame
            })
            .reduce(|mut acc, mut frame| {
                let max_height = std::cmp::max(acc.height(), frame.height());
                acc.expand_to_height(max_height);
                frame.expand_to_height(max_height);
                for row_index in 0..max_height {
                    acc[row_index].append(&mut frame[row_index]);
                }
                acc
            })
            .unwrap_or_else(|| vec![vec![]])
    }
}
