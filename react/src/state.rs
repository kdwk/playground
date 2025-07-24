use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

// pub struct State {
//     inner: HashMap<String, RefCell<dyn Any>>
// }
//
// impl State {
//     fn new() -> Self {
//         State {
//             inner: HashMap::new()
//         }
//     }
//     fn use_state<T: 'static, Mutation: Fn(&mut T)>(&mut self, key: impl Display, value: T) -> (Box<T>, Rc<dyn FnOnce(Mutation) + '_>) {
//         let key = key.to_string();
//         let val = RefCell::new(value);
//         let val = self.inner.entry(key.clone()).or_insert(val);
//         (self.inner.get(&key.clone()).unwrap().downcast().unwrap(), Rc::new(move |mutation| {
//             mutation(val.downcast_mut().unwrap());
//         }))
//     }
// }
//
// thread_local! {
//     pub static STATE: RefCell<State> = RefCell::new(State::new());
// }
//
// pub fn use_state<T: Any, Mutation: Fn(&mut T)>(key: impl Display, val: T) -> (Box<T>, Rc<dyn Fn(Mutation)>) {
//     let key = key.to_string();
//     let val = Box::new(val);
//     let mut ret_val = None;
//     STATE.with({
//         let key = key.clone();
//         move |state| {
//             ret_val = Some(state.borrow_mut().inner.entry(key).or_insert(val));
//         }
//     });
//     (ret_val.unwrap().downcast().unwrap(), Rc::new(|| {
//         STATE.with(move |state| {
//             state.borrow_mut().inner.entry(key).and_modify()
//         })
//     }))
// }