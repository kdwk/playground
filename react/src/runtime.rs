use std::rc::Rc;

use tokio::task::{self, JoinHandle};

use crate::{component::Component, render::render};

thread_local! {
    pub static RT: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
    pub static LOCAL: Rc<tokio::task::LocalSet> = Rc::new(tokio::task::LocalSet::new());
}

pub async fn run_app(app: Component) {
    let local = LOCAL.with(|local| local.clone());
    _ = local.run_until(render(app)).await;
}

pub fn go<T: 'static>(future: impl Future<Output = T> + 'static) -> JoinHandle<T> {
    // LOCAL.with(|local| local.spawn_local(future))
    LOCAL.with(|local| {
        local.spawn_local(future)
    })
    // task::spawn_local(future)
}

pub fn extract_or_none<T>(handle: &mut JoinHandle<T>) -> Option<T> {
    if handle.is_finished() {
        LOCAL.with(|local| {
            RT.with(|rt| {
                Some(local.block_on(rt, handle).unwrap())
            })
        })
    } else {
        None
    }
}

pub struct Context<'a> {
    runtime: &'a tokio::runtime::Runtime,
    local: &'a tokio::task::LocalSet,
}