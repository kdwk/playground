use std::time::Duration;

use tokio::time::sleep;

pub mod prelude {
    pub use super::wait;
}

pub async fn wait(secs: u64) {
    sleep(Duration::from_secs(secs)).await;
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
    }
}
