use std::{cell::RefCell, collections::VecDeque};

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
    pub(crate) static MESSAGE_QUEUE: RefCell<VecDeque<Message<'static>>> = RefCell::new(VecDeque::new());
}

pub fn send<T: 'static>(message: T) {
    MESSAGE_QUEUE.with_borrow_mut(|queue| queue.push_back(any(message)));
}

pub fn handle_messages(mut f: impl FnMut(&Message)) {
    let mut msgs = vec![];
    MESSAGE_QUEUE.with_borrow_mut(|queue| {
        while let Some(msg) = queue.pop_front() {
            msgs.push(msg);
        }
    });
    msgs.into_iter().for_each(|msg| f(&msg));
}
