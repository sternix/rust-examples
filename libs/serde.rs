/*

[dependencies]
anyhow = "1"
serde_json = "1"
serde = { version = "1.0", features = ["derive"] }

*/

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct ClusterMap;

fn main() -> Result<()> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    println!("cluster info: {:#?}", map);
    Ok(())
}