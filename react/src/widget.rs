use std::{cell::RefCell, rc::Rc};

use crossterm::event::KeyEvent;
use tokio::task::JoinHandle;

use crate::{component::prelude::*, prelude::Element, runtime::extract_or_none};

pub mod prelude {
    pub use super::{Widget, propagate};
}

pub struct Widget<State> {
    pub state: State,
    prev: Option<Component>,
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> Component>,
    on_keypress: Rc<dyn Fn(&mut Self, &KeyEvent)>,
    create_element: Rc<dyn Fn(&mut Self) -> Box<dyn Element>>,
}

impl<State> Widget<State>
where
    State: 'static,
{
    pub fn stateful(
        state: State,
        on_keypress: impl Fn(&mut Self, &KeyEvent) + 'static,
        builder: impl Fn(&State) -> Component + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            state: state,
            prev: None,
            needs_rebuild: true,
            builder: Box::new(builder),
            on_keypress: Rc::new(on_keypress),
            create_element: Rc::new(|this| this._build().borrow_mut().create_element()),
        }))
    }
    pub fn elemental(
        state: State,
        on_keypress: impl Fn(&mut Self, &KeyEvent) + 'static,
        create_element: impl Fn(&mut Self) -> Box<dyn Element> + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            state: state,
            prev: None,
            needs_rebuild: true,
            builder: Box::new(|_| panic!()),
            on_keypress: Rc::new(on_keypress),
            create_element: Rc::new(create_element),
        }))
    }
    fn _build(&mut self) -> Component {
        if !self.needs_rebuild
            && let Some(prev) = &self.prev
        {
            prev.clone()
        } else {
            let new_widget = (self.builder)(&self.state);
            self.prev = Some(new_widget.clone());
            self.needs_rebuild = false;
            new_widget
        }
    }
    pub fn set_state(&mut self, f: impl FnOnce(&mut State)) {
        f(&mut self.state);
        self.needs_rebuild = true;
    }
}

impl<T: 'static> Widget<(JoinHandle<T>, Option<T>)> {
    pub fn future(
        task: JoinHandle<T>,
        on_keypress: impl Fn(&mut Self, &KeyEvent) + 'static,
        builder: impl Fn(&Option<T>) -> Component + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            state: (task, None),
            prev: None,
            needs_rebuild: true,
            builder: Box::new(move |(_, result)| builder(result)),
            on_keypress: Rc::new(on_keypress),
            create_element: Rc::new(|this| {
                let (task, result) = &mut this.state;
                if let None = result {
                    match extract_or_none(task) {
                        Some(res) => {
                            *result = Some(res);
                            this.needs_rebuild = true;
                        }
                        None => {}
                    }
                }
                this._build().borrow_mut().create_element()
            }),
        }))
    }
}

impl<State> _Component for Widget<State> {
    fn create_element(&mut self) -> Box<dyn Element> {
        (self.create_element.clone())(self)
    }
    fn on_keypress(&mut self, event: &KeyEvent) {
        (self.on_keypress.clone())(self, event);
    }
}

pub fn propagate(this: &mut Widget<Vec<Component>>, event: &KeyEvent) {
    this.state
        .iter()
        .for_each(|child| child.borrow_mut().on_keypress(event));
}
