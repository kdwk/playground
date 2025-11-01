use crate::prelude::{Element, Frame, FrameExt, ensure_same_height};

pub mod prelude {
    pub use super::RowElement;
}

pub struct RowElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for RowElement {
    fn draw(&self) -> Frame {
        let mut children_frames = self
            .children
            .iter()
            .map(|child| child.draw())
            .collect::<Vec<_>>();
        ensure_same_height(&mut children_frames);
        let new_height = children_frames
            .get(0)
            .and_then(|f| Some(f.height()))
            .unwrap_or(0);
        (0..new_height)
            .map(|row_index| {
                children_frames
                    .iter()
                    .map(|f| f[row_index].clone())
                    .flatten()
                    .collect()
            })
            .collect()
    }
}
