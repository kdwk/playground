use std::{f32::consts::PI, time::Duration};

use tokio::{task::spawn as go, time::sleep};

pub mod prelude {
    pub use super::wait;
}

pub async fn wait(secs: u64) {
    sleep(Duration::from_secs(secs)).await;
}

pub async fn some_future() -> impl Future {
    async { wait(3).await }
}

pub async fn test() {
    let mut a: Vec<&dyn Future<Output = i32>> = vec![];
    a.push(&async {
        wait(4).await;
        5
    });
    a.push(&async { 6 });
    let mut b: &dyn Future<Output = i32>;
    for i in 0..10 {
        b = &async { 5 }
    }
}

pub mod test {
    use std::time::Duration;

    use crate::recipe::Discard;

    use super::prelude::*;
    use tokio::{task::spawn as go, time::sleep};

    pub async fn test1() {
        let future = go(async { wait(5) });
        while !future.is_finished() {
            println!("Still waitin'");
            sleep(Duration::from_millis(300)).await;
        }
        future.await.discard();
        println!("Waited 5s");
    }
    pub async fn test2() {
        let future = go(async {
            sleep(Duration::from_secs(1)).await;
            println!("World!");
        });
        println!("Hello");
        future.await.discard();
    }
    pub async fn test3() {
        let _ = go(async {});
        let a = 2;
    }
}
