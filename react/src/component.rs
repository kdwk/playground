use std::{cell::RefCell, rc::Rc};

use crate::prelude::{Element, Message};

pub mod prelude {
    pub use super::{_Component, Component};
}

pub trait _Component {
    fn id(&self) -> usize;
    fn create_element(&mut self) -> (bool, Box<dyn Element>);
    fn on_message(&mut self, event: &Message);
}

pub type Component = Rc<RefCell<dyn _Component>>;
