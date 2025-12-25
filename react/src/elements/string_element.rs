use crate::prelude::{DisplayList, Element, Operation, Point, Size};

pub mod prelude {
    pub use super::StringElement;
}

pub struct StringElement {
    pub s: String,
    pub cursor: Option<usize>,
}

impl Element for StringElement {
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        let mut offset = Point::default();
        for (i, c) in self.s.chars().enumerate() {
            if c != '\n' {
                display_list.0.push(Operation::PutChar(c));
            }
            if let Some(cursor) = self.cursor
                && cursor == i
            {
                if c == '\n' {
                    display_list.0.push(Operation::PutChar(' '));
                    display_list.0.push(Operation::DrawCursor);
                } else {
                    display_list.0.push(Operation::DrawCursor);
                }
            }
            if c == '\n' {
                offset.y += 1;
                offset.x = 0;
            } else {
                offset.x += 1;
            }
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
        if let Some(cursor) = self.cursor
            && cursor == self.s.len()
        {
            display_list.0.push(Operation::DrawCursor);
        }
    }
}
