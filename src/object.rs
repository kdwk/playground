use std::hash::Hash;

pub mod prelude {
    pub use super::{Enum, Object};
}

pub trait Object: Clone + PartialEq + Hash {
    fn class_name(&self) -> String;
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

pub trait Enum: Object {
    fn variants() -> Vec<Self>;
}
