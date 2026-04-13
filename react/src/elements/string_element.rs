use crate::prelude::{DisplayList, Element, Operation, Point, ProposedSize, Size};

pub mod prelude {
    pub use super::StringElement;
}

pub struct StringElement {
    pub s: String,
    pub cursor: Option<usize>,
}

impl Element for StringElement {
    fn propose_size(&self, proposed_constraints: ProposedSize) -> ProposedSize {
        match (proposed_constraints.x, proposed_constraints.y) {
            (Some(_), Some(_)) => proposed_constraints,
            (Some(x_constraint), None) => ProposedSize {
                x: Some(x_constraint),
                y: if x_constraint != 0 {
                    Some(count_lines(&self.s, x_constraint))
                } else {
                    None
                },
            },
            (None, Some(y_constraint)) => ProposedSize {
                x: None, // If parent has no x_constraint it makes more sense to let parent determine x than to write everything on a single line
                y: Some(y_constraint),
            },
            _ => proposed_constraints,
        }
    }
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

fn count_lines(s: &str, x_constraint: isize) -> isize {
    s.split("\n")
        .map(|line| (((line.len() as f64) / (x_constraint as f64)).ceil() as isize).max(1)) // The newline takes up at least one row
        .sum()
}
