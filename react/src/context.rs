use crossterm::event::KeyEvent;

pub struct Context<'a> {
    on_keypress_cbs: Vec<Box<dyn FnMut(&KeyEvent) + 'a>>,
}

impl<'a> Context<'a> {
    pub fn new() -> Self {
        Context {
            on_keypress_cbs: vec![],
        }
    }
    pub fn on_keypress(&mut self, f: impl FnMut(&KeyEvent) + 'a) {
        self.on_keypress_cbs.push(Box::new(f));
    }
    pub(crate) fn run_on_keypress_cbs(&mut self, keyevent: &KeyEvent) {
        for cb in self.on_keypress_cbs.iter_mut() {
            cb(keyevent);
        }
    }
}
