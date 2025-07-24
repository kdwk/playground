use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    marker::PhantomData,
    rc::Rc,
};
use std::fmt::Debug;
use std::ops::Deref;
use log::log;
use crate::{
    context::Context,
    widget::{DisplayList, Widget, widget},
};
use crate::log::log;

pub struct Stateful1<T: Clone, ChildBuilder: Fn(&Context, T, Rc<RefCell<T>>) -> Box<dyn Widget>> {
    next: Rc<RefCell<T>>,
    child: ChildBuilder
}

impl<T: Clone + Debug, ChildBuilder: Fn(&Context, T, Rc<RefCell<T>>) -> Box<dyn Widget>> Widget for Stateful1<T, ChildBuilder> {
    fn draw(&self, context: &Context) -> DisplayList {
        self.build(context).draw(context)
    }
    fn build(&self, context: &Context) -> Box<dyn Widget> {
        let val_ref = self.next.borrow();
        let val = val_ref.clone();
        log(format!("{val:?}"));
        drop(val_ref);
        (self.child)(context, val, self.next.clone())
    }
}

impl<T: Clone, ChildBuilder: Fn(&Context, T, Rc<RefCell<T>>) -> Box<dyn Widget>> Stateful1<T, ChildBuilder> {
    pub(crate) fn new(initial: T, child: ChildBuilder) -> Self {
        Self {
            next: Rc::new(RefCell::new(initial)),
            child
        }
    }
}

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
    T: 'static + Clone + Debug,
    Mutation: FnOnce(RefMut<T>),
    ChildBuilder: Fn(T, Box<dyn Fn(Mutation)>, &Context) -> Box<dyn Widget>,
> Widget for Stateful<T, Mutation, ChildBuilder>
{
    fn draw(&self, context: &Context) -> DisplayList {
        log("Stateful: did draw");
        if let Some(child) = self.child.borrow_mut().as_mut() {
            log(format!("Stateful: rebuild child: state at this point: {:?}", self.state.borrow()));
            *child = self.build(context);
            // if *(self.needs_rebuild.borrow()) {
            // }
        } else {
            log("Stateful: init child");
            *(self.child.borrow_mut()) = Some(self.build(context));
            // *(self.child) = Some(self.build(context));
            // self.child.replace(Some(self.build(context)));
            if let Some(_) = self.child.borrow_mut().as_mut() {
                log("Successfully inited child at this point");
            }
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
                log(format!("Ran mutation: {:?}", state.borrow()))
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
