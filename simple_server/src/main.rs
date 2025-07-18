use std::{error::Error, ops::RangeFrom, sync::Arc};

use axum::{Router, debug_handler, extract::State, routing::get, serve};
use extension::recipe::Log;
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let num = Arc::new(Mutex::new(0..));
    let app = Router::new().route("/nums", get(nums)).with_state(num);
    serve(TcpListener::bind("0.0.0.0:3000").await?, app).await?;
    Ok(())
}

#[debug_handler]
async fn nums(State(generator): State<Arc<Mutex<RangeFrom<u32>>>>) -> String {
    generator.lock().await.next().unwrap().log().to_string()
}
