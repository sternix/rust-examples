/*

[dependencies]
tokio = { version = "1", features = ["full"] }

*/

use std::time::Duration;
use tokio::join;
async fn count_and_wait(n: u64) -> u64 {
    println!("Starting {}", n);
    // ilginç olan burada thread duruyor,
    std::thread::sleep(Duration::from_millis(n * 100));
    println!("Returning {}", n);
    n
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Join runs multiple tasks concurrently and returns when they all
    // complete execution.
    join!(count_and_wait(1), count_and_wait(2), count_and_wait(3));
    Ok(())
}
