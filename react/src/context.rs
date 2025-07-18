use std::{any::Any, cell::RefCell, collections::VecDeque, rc::Rc};

pub(crate) struct _Context {
    pub(crate) objs: VecDeque<Rc<dyn Any>>,
}

pub struct Context {
    inner: RefCell<_Context>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Context {
            inner: RefCell::new(_Context {
                objs: VecDeque::new(),
            }),
        }
    }
    pub fn try_get<T: 'static>(&self) -> Option<Rc<T>> {
        self.inner
            .borrow()
            .objs
            .iter()
            .find(|obj| obj.is::<T>())
            .map(|obj| obj.clone().downcast().unwrap())
    }
    pub fn get<T: 'static>(&self) -> Rc<T> {
        self.try_get().unwrap()
    }
    pub fn inject<T: 'static>(&self, state: T) {
        self.inner.borrow_mut().objs.push_front(Rc::new(state));
    }
}
