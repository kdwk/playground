use crate::prelude::{DisplayList, Element, Operation, Point, Size};

pub mod prelude {
    pub use super::RowElement;
}

pub struct RowElement {
    pub children: Vec<Box<dyn Element>>,
}

impl Element for RowElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let child_width = constraint.x as usize / self.children.len();
        let mut x_offset = 0;
        for child in &self.children {
            let offset = Point {
                x: x_offset as isize,
                y: 0,
            };
            display_list.0.push(Operation::SetAnchor(offset));
            child.draw(
                Size {
                    x: child_width as isize,
                    y: constraint.y,
                },
                display_list,
            );
            display_list.0.push(Operation::SetAnchor(-offset));
            x_offset += child_width;
        }
    }
}
