use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    marker::PhantomData,
    rc::Rc,
};

use crate::{
    context::Context,
    widget::{DisplayList, Widget, widget},
};

pub struct Stateful<
    T: Clone,
    Mutation: FnOnce(RefMut<T>),
    ChildBuilder: Fn(T, Box<dyn Fn(Mutation)>, &Context) -> Box<dyn Widget>,
> {
    state: Rc<RefCell<T>>,
    builder: ChildBuilder,
    child: RefCell<Option<Box<dyn Widget>>>,
    needs_rebuild: Rc<RefCell<bool>>,
    phantom: PhantomData<Mutation>,
}

impl<
    T: 'static + Clone,
    Mutation: FnOnce(RefMut<T>),
    ChildBuilder: Fn(T, Box<dyn Fn(Mutation)>, &Context) -> Box<dyn Widget>,
> Widget for Stateful<T, Mutation, ChildBuilder>
{
    fn draw(&self, context: &Context) -> DisplayList {
        if let Some(child) = self.child.borrow_mut().as_mut() {
            *child = self.build(context);
            // if *(self.needs_rebuild.borrow()) {
            // }
        } else {
            self.child.replace(Some(self.build(context)));
        }
        // Child must exist at this point
        self.needs_rebuild.replace(false);
        self.child.borrow().as_ref().unwrap().draw(context)
    }
    fn build(&self, context: &Context) -> Box<dyn Widget> {
        let state = self.state.borrow().clone();
        let set_state = {
            let state = self.state.clone();
            let needs_rebuild = self.needs_rebuild.clone();
            Box::new(move |mutation: Mutation| {
                needs_rebuild.replace(true);
                mutation(state.borrow_mut());
            })
        };
        (self.builder)(state, set_state, context)
    }
}

impl<
    T: 'static + Clone,
    Mutation: FnOnce(RefMut<T>),
    ChildBuilder: Fn(T, Box<dyn Fn(Mutation)>, &Context) -> Box<dyn Widget>,
> Stateful<T, Mutation, ChildBuilder>
{
    pub fn new(init: T, builder: ChildBuilder) -> Self {
        Self {
            state: Rc::new(RefCell::new(init)),
            builder,
            child: RefCell::new(None),
            needs_rebuild: Rc::new(RefCell::new(true)),
            phantom: PhantomData,
        }
    }
}

impl<
    T: 'static + Clone + Default,
    Mutation: FnOnce(RefMut<T>),
    ChildBuilder: Fn(T, Box<dyn Fn(Mutation)>, &Context) -> Box<dyn Widget>,
> Stateful<T, Mutation, ChildBuilder>
{
    pub fn with_default(builder: ChildBuilder) -> Self {
        Self {
            state: Rc::new(RefCell::new(T::default())),
            builder,
            child: RefCell::new(None),
            needs_rebuild: Rc::new(RefCell::new(true)),
            phantom: PhantomData,
        }
    }
}
