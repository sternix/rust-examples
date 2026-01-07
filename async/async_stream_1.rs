/*
tokio = { version = "1", features = ["macros","rt-multi-thread","time"] }
futures = "0.3"
*/
use futures::{StreamExt, stream};
use rand::{Rng, thread_rng};
use std::time::Duration;

async fn compute_job(job: i64) -> i64 {
    let mut rng = thread_rng();
    let sleep_ms: u64 = rng.gen_range(0..10);
    tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

    job * job
}

async fn process_result(result: i64) {
    println!("{}", result);
}

#[tokio::main]
async fn main() {
    let jobs = 0..100;
    let concurrency = 42;

    stream::iter(jobs)
        .for_each_concurrent(concurrency, |job| async move {
            let result = compute_job(job).await;
            process_result(result).await;
        })
        .await;
}
