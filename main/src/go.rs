use std::future::Future;

pub mod prelude {
    pub use super::{Then, wait_for};
}

use tokio::task;

#[macro_export]
macro_rules! go {
    ( $( $x:expr )* $(;)* ) => {
        task::spawn(async move {
            $($x;)*
        })
    };
}

pub trait Then<T> {
    async fn then(self, run: impl FnOnce(T));
}

impl<T, F: Future<Output = T>> Then<T> for F {
    async fn then(self, run: impl FnOnce(T)) {
        run(self.await)
    }
}

pub fn wait_for<T>(future: impl Future<Output = T>) -> T {
    tokio::runtime::Builder::new_multi_thread()
        .build()
        .unwrap()
        .block_on(future)
}
