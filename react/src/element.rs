use crate::prelude::{DisplayList, ProposedSize, Constraint2};

pub mod prelude {
    pub use super::Element;
}

pub trait Element: Send {
    fn propose_size(&self, proposed_constraints: Constraint2) -> ProposedSize;
    fn draw(&self, constraint: Constraint2, display_list: &mut DisplayList);
}
