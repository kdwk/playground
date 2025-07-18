mod context;
mod counter;
mod dynamic_char;
mod element;
mod input;
mod map;
mod single_char;
mod stateful;
mod widget;

pub mod prelude {
    pub use super::{test, widget::prelude::*};
}

pub mod test {

    use crate::{counter::Counter, dynamic_char::DynamicChar, widget::prelude::*};

    pub async fn test() {
        run(Counter { val: 'a' }).await.unwrap();
    }
}
