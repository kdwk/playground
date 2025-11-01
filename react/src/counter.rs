use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent};
use react::{component::Component, widget::Widget};

use crate::number::number;

pub fn counter(i: i32) -> Rc<RefCell<dyn Component>> {
    Widget::stateful(
        i,
        |prev, event| match event {
            KeyEvent { code, .. } => match code {
                KeyCode::Char('+') => prev + 1,
                KeyCode::Char('-') => prev - 1,
                _ => *prev,
            },
        },
        |state| number(*state),
    )
}
