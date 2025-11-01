use crate::widgets::text::text;
use crate::{component::prelude::*, widget::Widget};

pub fn number(i: i32) -> Component {
    Widget::stateless(move || text(i.to_string()))
}
