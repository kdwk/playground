use crate::component::prelude::*;
use crate::widgets::text::text;

pub fn number(i: i32) -> Component {
    text(i.to_string())
}
