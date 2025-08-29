pub mod assertion;
pub mod input;
pub mod linked_list;
pub mod map;
pub mod mixture;
pub mod numbers;
pub mod plus;
pub mod quicksort;
pub mod recipe;
pub mod substr;
pub mod whoops;

pub mod prelude {
    pub use super::assertion::prelude::*;
    pub use super::input::prelude::*;
    pub use super::map;
    pub use super::mixture::prelude::*;
    pub use super::numbers::prelude::*;
    pub use super::plus::prelude::*;
    pub use super::quicksort::prelude::*;
    pub use super::recipe::prelude::*;
    pub use super::substr::prelude::*;
    pub use super::whoops::prelude::*;
    pub use super::{link, linked_list::prelude::*};
}
