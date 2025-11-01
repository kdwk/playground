use std::{cell::RefCell, rc::Rc};

use crossterm::event::KeyEvent;
use replace_with::replace_with_or_abort_and_return;

use crate::{component::prelude::*, prelude::Element};

pub mod prelude {}

pub struct Widget<State> {
    state: State,
    prev: Option<Rc<RefCell<dyn Component>>>,
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> Rc<RefCell<dyn Component>>>,
    on_keypress: Box<dyn Fn(&State, &KeyEvent) -> State>,
    create_element: Rc<dyn Fn(&mut Self) -> Box<dyn Element>>,
}

impl<State> Widget<State>
where
    State: 'static + PartialEq,
{
    pub fn stateless(
        builder: impl Fn() -> Rc<RefCell<dyn Component>> + 'static,
    ) -> Rc<RefCell<dyn Component>> {
        Rc::new(RefCell::new(Widget {
            state: (),
            prev: None,
            needs_rebuild: true,
            builder: Box::new(move |_| builder()),
            on_keypress: Box::new(|_, _| ()),
            create_element: Rc::new(|this| this._build().borrow_mut().create_element()),
        }))
    }
    pub fn stateful(
        state: State,
        on_keypress: impl Fn(&State, &KeyEvent) -> State + 'static,
        builder: impl Fn(&State) -> Rc<RefCell<dyn Component>> + 'static,
    ) -> Rc<RefCell<dyn Component>> {
        Rc::new(RefCell::new(Widget {
            state: state,
            prev: None,
            needs_rebuild: true,
            builder: Box::new(builder),
            on_keypress: Box::new(on_keypress),
            create_element: Rc::new(|this| this._build().borrow_mut().create_element()),
        }))
    }
    pub fn elemental(
        state: State,
        on_keypress: impl Fn(&State, &KeyEvent) -> State + 'static,
        create_element: impl Fn(&mut Self) -> Box<dyn Element> + 'static
    ) -> Rc<RefCell<dyn Component>> {
        Rc::new(RefCell::new(Widget {
            state: state,
            prev: None,
            needs_rebuild: true,
            builder: Box::new(|_| panic!()),
            on_keypress: Box::new(on_keypress),
            create_element: Rc::new(create_element)
        }))
    }
    fn _build(&mut self) -> Rc<RefCell<dyn Component>> {
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
}

impl<State> Component for Widget<State> {
    fn create_element(&mut self) -> Box<dyn Element> {
        (self.create_element.clone())(self)
    }
    fn on_keypress(&mut self, event: &KeyEvent) {
        // let prev = replace_with_or_abort_and_return(&mut self.state, |state| {
        //     (&state, (self.on_keypress)(&state, event))
        // });
        // (self.on_keypress)(&self.state, event);
    }
}
