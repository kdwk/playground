use crossterm::event::KeyCode;

use crate::{component::prelude::*, widget::Widget, widgets::text::text};

pub fn text_field() -> Component {
    Widget::stateful(
        "".to_string(),
        |this, event| match event.code {
            KeyCode::Enter => this.set_state(|state| state.push('\n')),
            KeyCode::Backspace => this.set_state(|state| _ = state.pop()),
            KeyCode::Char(c) => this.set_state(|state| state.push(c)),
            _ => {}
        },
        |buffer| text(buffer.clone()),
    )
}
