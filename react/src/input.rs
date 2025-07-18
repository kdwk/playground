use std::rc::Rc;

use crossterm::event::Event;

use crate::context::Context;

pub struct Input {
    pub event: Event,
}

impl Input {
    pub fn of(context: &Context) -> Rc<Self> {
        context.get()
    }
}

impl From<Event> for Input {
    fn from(value: Event) -> Self {
        Self { event: value }
    }
}
