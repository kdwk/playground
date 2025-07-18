use std::cell::{RefCell, RefMut};

use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::{
    context::Context,
    input::Input,
    single_char::SingleChar,
    stateful::Stateful,
    widget::{Widget, widget},
};

#[derive(Debug, Default)]
pub struct Counter {
    pub val: char,
}

impl Widget for Counter {
    fn build(&self, context: &Context) -> Box<dyn Widget> {
        widget(Stateful::new(self.val, |state, set_state, context| {
            let input = Input::of(context);
            match input.event {
                Event::Key(keyevent) => match keyevent.code {
                    KeyCode::Char(c) => set_state(move |mut state| *state = c),
                    // KeyCode::Char('-') => set_state(|mut state| *state -= 1),
                    _ => {}
                },
                _ => {}
            }
            widget(SingleChar { c: state })
        }))
    }
}
