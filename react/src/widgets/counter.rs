use crate::{component::prelude::*, widget::prelude::*};
use crossterm::event::KeyCode;

use crate::widgets::number::number;

pub fn counter(i: i32) -> Component {
    Widget::stateful(
        i,
        |this, event| match event.code {
            KeyCode::Char('+') => this.set_state(|i| *i += 1),
            KeyCode::Char('-') => this.set_state(|i| *i -= 1),
            _ => {}
        },
        |state| number(*state),
    )
}
