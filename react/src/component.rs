use std::{cell::RefCell, rc::Rc};

use crossterm::event::KeyEvent;

use crate::prelude::Element;

pub mod prelude {
    pub use super::{_Component, Component};
}

pub trait _Component {
    fn create_element(&mut self) -> Box<dyn Element>;
    fn on_keypress(&mut self, event: &KeyEvent);
}

pub type Component = Rc<RefCell<dyn _Component>>;
