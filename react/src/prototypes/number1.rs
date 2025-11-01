use react::{
    elements::{number_element::NumberElement, prelude::*},
    widget2::Widget,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Debug)]
pub struct Number {
    pub i: i32,
    prev: Option<Rc<RefCell<dyn Widget>>>,
    needs_rebuild: bool,
}

impl Number {
    pub fn new(i: i32) -> Self {
        Number {
            i,
            prev: None,
            needs_rebuild: true,
        }
    }
}

impl Widget for Number {
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
    fn create_element(&mut self) -> Box<dyn Element> {
        Box::new(NumberElement { i: self.i })
    }
    fn build(&self) -> Rc<RefCell<dyn Widget>> {
        unreachable!()
    }
}
