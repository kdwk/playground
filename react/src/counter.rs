use std::{cell::RefCell, rc::Rc};

use crossterm::event::{Event, KeyCode, KeyEvent};
use react::{
    hooks::{cb, on_keypress},
    widget::{Stateful, Widget, widget},
};

use crate::number::Number;

#[derive(Default, Debug)]
pub struct Counter {
    i: i32,
    prev: Option<Rc<RefCell<dyn Widget>>>,
    needs_rebuild: bool,
}

impl Counter {
    pub fn new(i: i32) -> Self {
        Counter {
            i,
            ..Default::default()
        }
    }
}

impl Widget for Counter {
    fn prev(&self) -> Option<Rc<RefCell<dyn Widget>>> {
        self.prev.clone()
    }
    fn set_prev(&mut self, prev: Rc<RefCell<dyn Widget>>) {
        self.prev = Some(prev);
    }
    fn needs_rebuild(&self) -> bool {
        self.needs_rebuild
    }
    fn set_needs_rebuild(&mut self, needs_rebuild: bool) {
        self.needs_rebuild = needs_rebuild;
    }
    fn on_keypress(&mut self, event: &Event) {
        if let Event::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Char('+') => self.set_state(|this| this.i += 1),
                KeyCode::Char('-') => self.set_state(|this| this.i -= 1),
                _ => {}
            }
        }
    }
    fn build(&self) -> Rc<RefCell<dyn Widget>> {
        widget(Number::new(self.i))
    }
}
