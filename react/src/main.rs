use react::{prelude::*, runtime::{RT, run_app}, widgets::{delayed::delayed, text_field::text_field}};
use stdext::prelude::*;

async fn run_local<T>(future: impl Future<Output = T>) -> T {
    let local_set = LocalSet::new();
    local_set.run_until(future).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    run_local(async {
        // render(text_field()).await?;
        // render(counter(1)).await?;
        render(delayed()).await?;
        Ok(())
    })
    .await
}

// fn main() {
//         rt.spawn(run_app(delayed()));
// }