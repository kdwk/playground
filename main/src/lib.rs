pub mod arithmetic;
pub mod map;
pub mod numbers;
pub mod quicksort;
pub mod recipe;
pub mod set;
pub mod whoops;

pub mod prelude {
    pub use super::arithmetic::prelude::*;
    pub use super::numbers::prelude::*;
    pub use super::quicksort::{self, prelude::*};
    pub use super::recipe::prelude::*;
    pub use super::whoops::prelude::*;
    pub use super::{map, set};
}
