use core::{future::Future, ops::FnOnce, pin::Pin, task::Poll, todo};
use std::{task::Waker, thread, time::Duration};

use tokio::{
    task,
    time::{Instant, sleep},
};

pub mod prelude {
    pub use super::{Timer, wait};
}

pub async fn wait(secs: u64) {
    sleep(Duration::from_secs(secs)).await;
}

pub struct Timer {
    start: Instant,
    duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Self {
        Self {
            start: Instant::now(),
            duration,
        }
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // println!("Polled");
        let time = Instant::now();
        if time - self.start < self.duration {
            let w = cx.waker().clone();
            let duration = self.duration.clone();
            task::spawn(async move {
                sleep(duration).await;
                w.wake_by_ref();
            });
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

struct AndThen<'a, T, O, Promise: Future<Output = T>, F: FnOnce(T) -> O> {
    inner: Pin<&'a mut Promise>,
    closure: F,
}

impl<'a, T, O, Promise: Future<Output = T>, F: FnOnce(T) -> O> Future
    for AndThen<'a, T, O, Promise, F>
{
    type Output = O;
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> Poll<Self::Output> {
        todo!()
        // match self.inner.as_mut().poll(cx) {
        //     Poll::Ready(val) => todo!(),
        //     Poll::Pending => todo!(),
        // }
    }
}

pub mod test {
    use std::time::Duration;

    use tokio::{sync::mpsc, task, time::sleep};

    use crate::recipe::Discard;

    use super::prelude::*;

    pub async fn test1() {
        let future = task::spawn(wait(5));
        while !future.is_finished() {
            println!("Still waiting");
            sleep(Duration::from_millis(500)).await;
        }
        future.await.discard();
        println!("Finished waiting 5 seconds");
    }

    pub async fn test2() {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let future = task::spawn(async move {
            for i in 1..=10 {
                sender.send(i).unwrap();
                if i < 10 {
                    sleep(Duration::from_millis(500)).await;
                }
            }
        });
        while let Some(i) = receiver.recv().await {
            println!("{i}");
        }
        future.await.discard();
    }

    pub async fn test3() {
        let timer = task::spawn(Timer::new(Duration::from_secs(5)));
        timer.await.discard();
        println!("After 5 seconds");
    }
}
