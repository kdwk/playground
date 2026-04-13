use crate::prelude::{Color, Element, ProposedSize, Size, DisplayList};

pub mod prelude {

}

pub struct ColorElement {
    pub color: Color,
    pub child: Box<dyn Element>
}

impl Element for ColorElement {
    fn propose_size(&self, proposed_constraints: ProposedSize) -> ProposedSize {
        self.child.propose_size(proposed_constraints)
    }
    fn draw(&self, constraint: Size, display_list: &mut DisplayList) {
        
    }
}