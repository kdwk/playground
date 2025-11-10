use crossterm::event::{KeyCode, KeyEvent};
use stdext::prelude::AnythingExt;

use crate::{component::prelude::*, widget::Widget, widgets::text::text};

pub fn text_field() -> Component {
    Widget::stateful(
        "".to_string(),
        |this, msg| {
            msg.case::<KeyEvent>(|event| match event.code {
                KeyCode::Enter => this.set_state(|buffer| buffer.push('\n')),
                KeyCode::Backspace => this.set_state(|buffer| _ = buffer.pop()),
                KeyCode::Char(c) => this.set_state(|buffer| buffer.push(c)),
                _ => {}
            });
        },
        |buffer| text(buffer.clone()),
    )
}
