use crate::prelude::{DisplayList, Element, ProposedSize, Operation, Size};

pub mod prelude {
    pub use super::CharElement;
}

pub struct CharElement {
    pub c: char,
}

impl Element for CharElement {
    fn propose_size(&self, proposed_constraints: ProposedSize) -> ProposedSize {
        ProposedSize {x: Some(1), y: Some(1)}.min(proposed_constraints)
    }
    fn draw(&self, _constraint: Size, display_list: &mut DisplayList) {
        display_list.0.push(Operation::PutChar(self.c));
    }
}
