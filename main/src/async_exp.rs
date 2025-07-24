use std::{f32::consts::PI, task::Poll, time::Duration};

use tokio::{
    task::spawn_local as go,
    time::{Sleep, sleep},
};

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
    use std::sync::Arc;
    use std::{cell::RefCell, sync::atomic::AtomicU32, time::Duration};
    use std::rc::Rc;
    use crate::recipe::Discard;

    use super::prelude::*;
    use anyhow::Result;
    use reqwest::get;
    use tokio::sync::Mutex;
    use tokio::{
        task::{JoinSet, spawn_local as go},
        time::sleep,
    };

    #[derive(Clone)]
    struct A {
        i: String,
    }

    struct Data {
        a: String,
    }

    pub async fn test1() {
        let future = go(async { wait(5) });
        while !future.is_finished() {
            println!("Still waitin'");
            sleep(Duration::from_millis(100)).await;
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
        let mut a = A {
            i: "hi".to_string(),
        };
        let future = go({
            let a = a.clone();
            async move {
                sleep(Duration::from_secs(5)).await;
                a.i + "ya"
            }
        });
        a.i = future.await.unwrap();
        println!("{}", a.i);
    }
    pub async fn test4() {
        let future = go(sleep(Duration::from_secs(5)));
        while !future.is_finished() {
            println!("Still waitin'");
            sleep(Duration::from_secs(1)).await;
        }
        println!("After 5s!");
    }
    pub async fn test5() -> Result<()> {
        let i;
        let new_value = go(async {
            sleep(Duration::from_secs(3)).await;
            1
        });
        i = new_value.await?;
        println!("{i}");
        Ok(())
    }
    pub async fn test6() -> u32 {
        let tasks = (0..100)
            .map(|_| async {
                get("http://localhost:3000/nums")
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .parse()
                    .unwrap()
            })
            .collect::<JoinSet<u32>>();
        tasks.join_all().await.iter().sum()
    }
    pub async fn test7() -> usize {
        let tasks = (0..10)
            .map(|_| async {
                get("https://wikipedia.org")
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap()
                    .chars()
                    .count()
            })
            .collect::<JoinSet<_>>();
        tasks.join_all().await.iter().sum()
    }
    pub async fn test8() {
        let data = Data {
            a: "something".to_string(),
        };
        let c1 = go(async move {
            for i in (0..3).rev() {
                println!("{}", i + 1);
                sleep(Duration::from_secs(1)).await;
            }
            data.a.chars().nth(0).unwrap()
        });
        println!("{}", c1.await.unwrap());
    }
    pub async fn test9() {
        let mut a = [1, 2, 3, 4];
        let c = go({
            let a = a.clone();
            async move {
                sleep(Duration::from_secs(3)).await;
                a[2] + 1
            }
        });
        a[2] = c.await.unwrap();
    }
    // pub async fn test10() {
    //     let mut a = Arc::new(4);
    //     go({
    //         let a = a.clone();
    //         async move {
    //             sleep(Duration::from_secs(3)).await;
    //             *a += 2;
    //         }
    //     }).await.discard();
    //     println!("{a}");
    // }
}
