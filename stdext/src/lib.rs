pub mod linked_list;
pub mod map;
pub mod mixture;
pub mod numbers;
pub mod plus;
pub mod quicksort;
pub mod recipe;
pub mod whoops;
pub mod assertion;
pub mod substr;

pub mod prelude {
    pub use super::map;
    pub use super::mixture::prelude::*;
    pub use super::numbers::prelude::*;
    pub use super::plus::prelude::*;
    pub use super::quicksort::prelude::*;
    pub use super::recipe::prelude::*;
    pub use super::whoops::prelude::*;
    pub use super::{link, linked_list::prelude::*};
    pub use super::assertion::prelude::*;
    pub use super::substr::prelude::*;
}
