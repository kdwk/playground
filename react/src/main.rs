use react::prelude::*;
use stdext::prelude::*;

use crate::counter::counter;

mod counter;
mod number;

async fn run_local<T>(future: impl Future<Output = T>) -> T {
    let local_set = LocalSet::new();
    local_set.run_until(future).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    run_local(async {
        render(counter(1)).await?;
        Ok(())
    })
    .await
}
