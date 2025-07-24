use std::cell::{RefCell, RefMut};

use crossterm::event::{Event, KeyCode, KeyEvent};
use crate::log::log;
use crate::{
    context::Context,
    input::Input,
    single_char::SingleChar,
    stateful::Stateful,
    widget::{Widget, widget},
};
use crate::stateful::Stateful1;

#[derive(Debug, Default)]
pub struct Counter {
    pub val: char,
}

impl Widget for Counter {
    fn build(&self, context: &Context) -> Box<dyn Widget> {
        widget(Stateful1::new(self.val, |context, state, next| {
            // log(state);
            let input = Input::of(context);
            match input.event {
                Event::Key(keyevent) => match keyevent.code {
                    KeyCode::Char(c) => *next.borrow_mut() = c,
                    // KeyCode::Char('-') => set_state(|mut state| *state -= 1),
                    _ => {}
                },
                _ => {}
            }
            widget(SingleChar { c: state })
        }))
    }
}
