use crossterm::event::KeyEvent;

use crate::prelude::Element;

pub mod prelude {
    pub use super::Component;
}

pub trait Component {
    fn create_element(&mut self) -> Box<dyn Element>;
    fn on_keypress(&mut self, event: &KeyEvent);
}