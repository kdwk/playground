use crate::prelude::{Direction, DisplayList, Element, Operation, Point, Size};

pub mod prelude {
    pub use super::StringElement;
}

pub struct StringElement {
    pub s: String,
}

impl Element for StringElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let mut offset = Point::default();
        for c in self.s.chars() {
            display_list.0.push(Operation::PutChar(c));
            offset.x += 1;
            if offset.x >= constraint.x {
                offset.y += 1;
                offset.x = 0;
                if offset.y >= constraint.y {
                    display_list.0.push(Operation::PutChar('…'));
                    break;
                }
            }
            display_list.0.push(Operation::MoveTo(offset));
        }
    }
}
