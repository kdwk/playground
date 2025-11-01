use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crossterm::event::Event;

use crate::{context::Context, elements::prelude::*};

pub trait Widget: Debug {
    fn prev(&self) -> Option<Rc<RefCell<dyn Widget>>>;
    fn set_prev(&mut self, prev: Rc<RefCell<dyn Widget>>);
    fn needs_rebuild(&self) -> bool;
    fn set_needs_rebuild(&mut self, needs_rebuild: bool);
    fn create_element(&mut self) -> Box<dyn Element> {
        self._build().borrow_mut().create_element()
    }
    fn on_keypress(&mut self, event: &Event) {}
    fn _build(&mut self) -> Rc<RefCell<dyn Widget>> {
        if !self.needs_rebuild()
            && let Some(prev) = self.prev()
        {
            prev.clone()
        } else {
            let new_widget = self.build();
            self.set_prev(new_widget);
            self.set_needs_rebuild(false);
            self.prev().unwrap()
        }
    }
    fn build(&self) -> Rc<RefCell<dyn Widget>>;
}

#[inline]
pub fn widget(w: impl Widget + 'static) -> Rc<RefCell<dyn Widget>> {
    Rc::new(RefCell::new(w))
}

#[derive(Default)]
pub struct Component {
    prev: Option<Rc<RefCell<dyn Widget>>>,
    needs_rebuild: bool,
    mutations: Vec<Box<dyn Fn(&mut dyn Widget)>>,
}

impl Component {
    // fn consume_mutations(&mut )
}

pub trait Stateful {
    fn set_state(&mut self, f: impl FnOnce(&mut Self));
}

impl<T: Widget> Stateful for T {
    #[inline]
    fn set_state(&mut self, f: impl FnOnce(&mut Self)) {
        f(self);
        self.set_needs_rebuild(true);
    }
}
