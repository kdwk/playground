use crate::{
    element::ensure_same_width,
    prelude::{Element, Frame},
};

pub struct ColumnElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for ColumnElement {
    fn draw(&self) -> Frame {
        let mut children_frames = self
            .children
            .iter()
            .map(|child| child.draw())
            .collect::<Vec<_>>();
        ensure_same_width(&mut children_frames);
        children_frames.into_iter().flatten().collect()
    }
}
