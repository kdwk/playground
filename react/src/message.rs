use std::cell::RefCell;

use stdext::prelude::{Anything, any};

pub mod prelude {
    pub use super::{
        Message, MessageFlow, MessageFlow::Intercept, MessageFlow::Propagate, handle_messages, send,
    };
}

pub type Message<'a> = Anything<'a>;

#[derive(Debug, Clone, Copy, Hash)]
pub enum MessageFlow {
    Propagate,
    Intercept,
}

impl Default for MessageFlow {
    fn default() -> Self {
        Self::Propagate
    }
}

thread_local! {
    pub(crate) static MESSAGE_QUEUE: RefCell<Vec<Message<'static>>> = RefCell::new(vec![]);
}

pub fn send<T: 'static>(message: T) {
    MESSAGE_QUEUE.with_borrow_mut(|queue| queue.push(any(message)));
}

pub fn handle_messages(f: impl Fn(&Message)) {
    MESSAGE_QUEUE.with_borrow_mut(|queue| {
        while let Some(msg) = queue.pop() {
            f(&msg);
        }
    });
}
