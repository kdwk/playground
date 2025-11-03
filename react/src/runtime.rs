use std::fmt::Display;

use documents::prelude::*;
use tokio::task::{self, JoinHandle};

pub mod prelude {
    pub use super::{extract_or_none, go, go_block, log, wait_for};
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

pub fn extract_or_none<T>(handle: &mut JoinHandle<T>) -> Option<T> {
    if handle.is_finished() {
        RT.with(|rt| Some(rt.block_on(handle).unwrap()))
    } else {
        None
    }
}

pub fn wait_for<T>(handle: &mut JoinHandle<T>) -> Result<T, task::JoinError> {
    RT.with(|rt| rt.block_on(handle))
}

pub fn log<T: Display>(s: T) -> T {
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
