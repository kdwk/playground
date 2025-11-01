use crate::elements::prelude::*;
use std::rc::Rc;

pub struct Component<State> {
    prev: Option<Rc<Component<State>>>,
    needs_rebuild: bool,
    state: State,
    on_keypress: Option<Box<dyn Fn(&mut State)>>,
    mutation: Option<Box<dyn Fn(&mut State)>>,
    builder: Box<dyn Fn(&State) -> Box<dyn Element>>,
}

pub fn component<State>(
    state: State,
    builder: impl Fn(&State) -> Box<dyn Element> + 'static,
) -> Component<State> {
    Component {
        prev: None,
        needs_rebuild: true,
        state: state,
        on_keypress: None,
        mutation: None,
        builder: Box::new(builder),
    }
}

impl<State: 'static> Component<State> {
    fn build(&self) -> Box<dyn Element> {
        (self.builder)(&self.state)
    }
    fn next(&mut self) {
        let mutation = self.mutation.take();
        if let Some(mutation) = mutation {
            mutation(&mut self.state);
        }
    }
    fn set_state(&mut self, f: impl Fn(&mut State) + 'static) {
        let mutation = self.mutation.take();
        if let Some(mutation) = mutation {
            self.mutation = Some(Box::new(move |state| {
                f(state);
                mutation(state);
            }));
        } else {
            self.mutation = Some(Box::new(f));
        }
    }
    fn did_keypress(&mut self) {
        // if let Some(on_keypress) = self.on_keypress {
        //     on_keypress(&mut self.state);
        // }
    }
}
