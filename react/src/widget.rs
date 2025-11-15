use std::{cell::RefCell, rc::Rc};

use tokio::task::JoinHandle;

use crate::{
    component::prelude::*, message::prelude::*, prelude::Element, runtime::extract_or_none,
};

pub mod prelude {
    pub use super::{Widget, propagate};
}

thread_local! {
    pub(crate) static COUNTER: RefCell<usize> = RefCell::new(0);
}

pub fn uid() -> usize {
    COUNTER.with(|counter| {
        let id = *counter.borrow();
        *counter.borrow_mut() += 1;
        id
    })
}

pub struct Widget<State> {
    id: usize,
    pub state: State,
    prev: Option<Component>,
    needs_rebuild: bool,
    builder: Box<dyn Fn(&State) -> Component>,
    on_message: Rc<dyn Fn(&mut Self, &Message)>,
    create_element: Rc<dyn Fn(&mut Self) -> (bool, Box<dyn Element>)>,
}

impl<State> Widget<State>
where
    State: 'static,
{
    pub fn stateful(
        state: State,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&State) -> Component + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            prev: None,
            needs_rebuild: true,
            builder: Box::new(builder),
            on_message: Rc::new(move |this, msg| {
                if let Propagate = on_message(this, msg)
                    && let Some(prev) = &this.prev
                {
                    prev.borrow_mut().on_message(msg);
                }
            }),
            create_element: Rc::new(|this| {
                let (did_rebuild, widget) = this._build();
                let (did_child_rebuild, child_element) = widget.borrow_mut().create_element();
                (did_rebuild || did_child_rebuild, child_element)
            }),
        }))
    }
    pub fn elemental(
        state: State,
        on_message: impl Fn(&mut Self, &Message) + 'static,
        create_element: impl Fn(&mut Self) -> (bool, Box<dyn Element>) + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: state,
            prev: None,
            needs_rebuild: true,
            builder: Box::new(|_| panic!()),
            on_message: Rc::new(on_message),
            create_element: Rc::new(create_element),
        }))
    }
    fn _build(&mut self) -> (bool, Component) {
        if !self.needs_rebuild
            && let Some(prev) = &self.prev
        {
            (false, prev.clone())
        } else {
            let new_widget = (self.builder)(&self.state);
            self.prev = Some(new_widget.clone());
            self.needs_rebuild = false;
            (true, new_widget)
        }
    }
    #[inline]
    pub fn set_state(&mut self, f: impl FnOnce(&mut State)) {
        f(&mut self.state);
        self.needs_rebuild = true;
    }
}

impl<T: 'static> Widget<(JoinHandle<T>, Option<T>)> {
    pub fn future(
        task: JoinHandle<T>,
        on_message: impl Fn(&mut Self, &Message) -> MessageFlow + 'static,
        builder: impl Fn(&Option<T>) -> Component + 'static,
    ) -> Component {
        Rc::new(RefCell::new(Widget {
            id: uid(),
            state: (task, None),
            prev: None,
            needs_rebuild: true,
            builder: Box::new(move |(_, result)| builder(result)),
            on_message: Rc::new(move |this, msg| {
                if let Propagate = on_message(this, msg)
                    && let Some(prev) = &this.prev
                {
                    prev.borrow_mut().on_message(msg);
                }
            }),
            create_element: Rc::new(|this| {
                let (task, result) = &mut this.state;
                if let None = result {
                    if let Some(res) = extract_or_none(task) {
                        *result = Some(res);
                        this.needs_rebuild = true;
                    }
                }
                let (did_rebuild, widget) = this._build();
                let (did_child_rebuild, child_element) = widget.borrow_mut().create_element();
                (did_rebuild || did_child_rebuild, child_element)
            }),
        }))
    }
}

// impl<T: 'static, Streamable: Stream<Item = T>> Widget<Stream> {
//     pub fn stream(
//         stream: Streamable,
//         on_keypress: impl Fn(&mut Self, &KeyEvent) + 'static,
//         builder: impl Fn(&Option<T>) -> Component + 'static
//     ) -> Component {
//         Rc::new(Cell::new(Widget {
//             state: stream,
//             prev: None,
//             needs_rebuild: true,
//             builder:
//         }))
//     }
// }

impl<State> _Component for Widget<State> {
    #[inline]
    fn id(&self) -> usize {
        self.id
    }
    #[inline]
    fn create_element(&mut self) -> (bool, Box<dyn Element>) {
        (self.create_element.clone())(self)
    }
    #[inline]
    fn on_message(&mut self, event: &Message) {
        (self.on_message.clone())(self, event);
    }
}

pub fn propagate(this: &mut Widget<Vec<Component>>, event: &Message) {
    this.state
        .iter()
        .for_each(|child| child.borrow_mut().on_message(event));
}
