use react::{prelude::*, widget2::widget};
use stdext::prelude::*;

use crate::{counter::Counter, number::Number};

mod counter;
mod elements;
mod number;

async fn run_local<T>(future: impl Future<Output = T>) -> T {
    let local_set = LocalSet::new();
    local_set.run_until(future).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    run_local(async {
        render(widget(Counter::new(1))).await?;
        Ok(())
    })
    .await
}
