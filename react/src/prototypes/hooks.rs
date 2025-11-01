use std::cell::RefCell;

use crossterm::event::Event;

thread_local! {
    pub static ON_KEYPRESS_CBS: RefCell<Vec<Box<dyn FnMut(&Event)>>> = RefCell::new(vec![]);
}

pub fn on_keypress(f: impl FnMut(&Event) + 'static) {
    ON_KEYPRESS_CBS.with(|on_keypress_cbs| {
        let mut on_keypress_cbs = on_keypress_cbs.borrow_mut();
        on_keypress_cbs.push(Box::new(f));
    })
}

pub fn cb(f: impl FnMut() + 'static) -> Box<dyn FnMut()> {
    Box::new(f)
}
