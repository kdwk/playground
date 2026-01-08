use std::fmt::Display;

use tokio::{
    sync::mpsc::UnboundedReceiver,
    task::{self, JoinError, JoinHandle},
};

pub mod prelude {
    pub use super::{Task, go, go_block, log, wait_for};
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

impl<T> Drop for Task<T> {
    fn drop(&mut self) {
        match self {
            Self::Running(join_handle) => join_handle.abort(),
            _ => {}
        }
    }
}

pub struct Stream<T, TaskRet> {
    pub task: Task<TaskRet>,
    pub receiver: UnboundedReceiver<T>,
    pub current: Option<T>,
}

impl<T, TaskRet> Stream<T, TaskRet> {
    pub fn check(&mut self) -> bool {
        let did_task_status_change = self.task.check();
        if self.receiver.is_empty() {
            did_task_status_change
        } else {
            self.current = self.receiver.blocking_recv();
            true
        }
    }
    pub fn current(&self) -> Option<&T> {
        self.current.as_ref()
    }
}

impl<T, TaskRet> Drop for Stream<T, TaskRet> {
    fn drop(&mut self) {
        match &mut self.task {
            Task::Running(handle) => handle.abort(),
            _ => {}
        }
    }
}

#[inline]
pub fn wait_for<T>(handle: &mut JoinHandle<T>) -> Result<T, task::JoinError> {
    RT.with(|rt| rt.block_on(handle))
}

pub fn log<T: Display>(s: T) -> T {
    eprintln!("{s}");
    s
}
