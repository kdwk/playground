use crate::prelude::{Element, ProposedSize};

pub mod prelude {
    pub use super::SizedElement;
}

pub struct SizedElement {
    pub size: ProposedSize,
    pub child: Box<dyn Element>,
}

impl Element for SizedElement {
    fn propose_size(&self, proposed_constraints: ProposedSize) -> ProposedSize {
        match (self.size.x, self.size.y) {
            (Some(_), Some(_)) => self.size,
            (x @ Some(_), None) => ProposedSize {
                x,
                y: self
                    .child
                    .propose_size(ProposedSize {
                        x,
                        y: proposed_constraints.y,
                    })
                    .y,
            },
            (None, y @ Some(_)) => ProposedSize {
                x: self.child.propose_size(ProposedSize {
                    x: proposed_constraints.x,
                    y,
                }).x,
                y,
            },
            _ => self.child.propose_size(proposed_constraints)
        }
    }
    fn draw(
        &self,
        constraint: crate::prelude::Size,
        display_list: &mut crate::prelude::DisplayList,
    ) {
        self.child.draw(constraint, display_list);
    }
}
