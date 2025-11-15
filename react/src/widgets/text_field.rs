use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};
use stdext::prelude::*;

use crate::{component::prelude::*, message::prelude::*, widget::Widget, widgets::text::text};

pub fn text_field(buffer: Rc<RefCell<String>>) -> Component {
    Widget::stateful(
        buffer,
        |this, msg| {
            switch(msg).case(|event: &KeyEvent| match event.code {
                KeyCode::Enter => this.set_state(|buffer| buffer.borrow_mut().push('\n')),
                KeyCode::Backspace => this.set_state(|buffer| _ = buffer.borrow_mut().pop()),
                KeyCode::Char(c) => this.set_state(|buffer| buffer.borrow_mut().push(c)),
                _ => {}
            });
            Intercept
        },
        |buffer| text(buffer.borrow().clone()),
    )
}
