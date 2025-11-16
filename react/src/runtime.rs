use std::fmt::Display;

use documents::prelude::*;
use tokio::task::{self, JoinError, JoinHandle};

pub mod prelude {
    pub(crate) use super::log;
    pub use super::{Task, go, go_block, wait_for};
}

thread_local! {
    pub static RT: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
}

pub fn go<T: 'static + Send + Sync>(
    future: impl Future<Output = T> + Send + Sync + 'static,
) -> JoinHandle<T> {
    RT.with(|rt| rt.spawn(future))
}

pub fn go_block<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    RT.with(|rt| rt.spawn_blocking(f))
}

#[derive(Debug)]
pub enum Task<T> {
    Running(JoinHandle<T>),
    Done(T),
    Err(JoinError),
}

impl<T> Task<T> {
    pub fn check(&mut self) -> bool {
        match self {
            Self::Running(join_handle) => {
                if join_handle.is_finished() {
                    match wait_for(join_handle) {
                        Ok(res) => *self = Task::Done(res),
                        Err(err) => *self = Task::Err(err),
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[inline]
pub fn wait_for<T>(handle: &mut JoinHandle<T>) -> Result<T, task::JoinError> {
    RT.with(|rt| rt.block_on(handle))
}

pub(crate) fn log<T: Display>(s: T) -> T {
    with(
        &[Document::at_path(
            "./log.txt",
            "log",
            Create::OnlyIfNotExists,
        )],
        |mut d| {
            d["log"].append(s.to_string().as_bytes())?;
            Ok(())
        },
    );
    s
}
