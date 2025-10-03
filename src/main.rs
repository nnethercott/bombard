use std::time::Duration;

use futures::future;
use tokio::time::sleep;

async fn bleh_to_ok(s: u64) -> Result<(), String> {
    sleep(Duration::from_secs(s)).await;
    Err("should run both then fail".into())
}

#[tokio::main]
async fn main() {
    let it = [bleh_to_ok(1), bleh_to_ok(2)]
        .into_iter()
        .map(|fut| Box::pin(fut));

    future::select_ok(it).await.unwrap();
}
