use crate::prelude::{Color, Element, ProposedSize, Size, DisplayList, Constraint2};

pub mod prelude {

}

pub struct ColorElement {
    pub color: Color,
    pub child: Box<dyn Element>
}

impl Element for ColorElement {
    fn propose_size(&self, proposed_constraints: Constraint2) -> ProposedSize {
        self.child.propose_size(proposed_constraints)
    }
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList) {
        
    }
}