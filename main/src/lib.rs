pub mod numbers;
pub mod quicksort;
pub mod recipe;

pub mod prelude {
    pub use super::numbers::prelude::*;
    pub use super::quicksort::prelude::*;
    pub use super::recipe::prelude::*;
}
