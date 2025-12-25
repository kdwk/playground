use std::fmt::Display;

use crate::{
    prelude::{Component, StringElement},
    widget::Widget,
};

pub fn text_cursor(s: impl Display + 'static, cursor: Option<usize>) -> Component {
    Widget::elemental(
        (s.to_string(), cursor),
        |_, _| (),
        |this| {
            (
                false,
                Box::new(StringElement {
                    s: this.state.0.clone(),
                    cursor: this.state.1,
                }),
            )
        },
    )
}
