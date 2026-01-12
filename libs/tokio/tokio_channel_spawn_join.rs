// https://github.com/tokio-rs/tokio/issues/2401

/*
[dependencies]
tokio = {version="1", features=["macros","rt-multi-thread","sync"]}
futures = "0.3"
*/

//use futures;
use tokio;

use std::time::Instant;

#[tokio::main]
async fn main() {
    let spawn_time = Instant::now();
    let mut sub_tasks = Vec::with_capacity(1_000_000);
    let (tx, mut rx) = tokio::sync::mpsc::channel(1000);
    // Spawn & send
    for _ in 0..1_000_000 {
        let sender = tx.clone();
        let handle = tokio::spawn(async move {
            sender.send(0).await.unwrap();
        });
        sub_tasks.push(handle);
    }
    // Receive
    for _ in 0..1_000_000 {
        let _ = rx.recv().await;
    }
    println!(
        "Task spawned and received message: {}ms",
        spawn_time.elapsed().as_millis()
    );

    let join_all_time = Instant::now();

    //futures::future::join_all(sub_tasks).await;
    // yerine bunu kullan
    for handle in sub_tasks {
        handle.await.unwrap();
    }

    println!(
        "Joining all handles: {}ms",
        join_all_time.elapsed().as_millis()
    );
}
