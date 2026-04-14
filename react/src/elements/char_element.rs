use crate::prelude::{Constraint2, DisplayList, Element, Operation, Pixel, ProposedSize, Size};

pub mod prelude {
    pub use super::CharElement;
}

pub struct CharElement {
    pub c: char,
}

impl Element for CharElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> ProposedSize {
        Constraint2 {
            x: Some(1),
            y: Some(1),
        }
        .min(proposed_constraints)
        .propose_as_pixels()
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        match (constraint.x, constraint.y) {
            (Some(x), Some(y)) if x == 0 || y == 0 => {}
            _ => display_list.0.push(Operation::PutChar(self.c)),
        }
    }
}
