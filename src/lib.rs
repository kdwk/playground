pub mod numbers;
pub mod quicksort;

pub mod prelude {
    pub use super::numbers::prelude::*;
    pub use super::quicksort::{self, prelude::*};
}
