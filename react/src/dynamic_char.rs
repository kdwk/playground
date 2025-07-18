use crossterm::event::{Event, KeyCode};

use crate::{
    context::Context,
    input::Input,
    single_char::SingleChar,
    widget::{prelude::*, widget},
};

pub struct DynamicChar;

impl Widget for DynamicChar {
    fn build(&self, context: &Context) -> Box<dyn Widget> {
        let input = Input::of(context);
        if let Event::Key(event) = input.event {
            if let KeyCode::Char(c) = event.code {
                return widget(SingleChar { c });
            }
        }
        widget(SingleChar::default())
    }
}
