use crate::prelude::{DisplayList, ProposedSize, Size};

pub mod prelude {
    pub use super::Element;
}

pub trait Element: Send {
    fn propose_size(&self, proposed_constraints: ProposedSize) -> ProposedSize;
    fn draw(&self, constraint: Size, display_list: &mut DisplayList);
}
