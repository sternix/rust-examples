/*

[dependencies]
tokio-postgres = "0.7"
tokio = {version = "1", features = ["full"]}

*/

use tokio_postgres::{Config, NoTls};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = Config::new()
        .host("localhost")
        .user("test")
        .password("test")
        .dbname("test")
        .connect(NoTls)
        .await?;

    tokio::spawn(async move { connection.await.unwrap() });

    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    Ok(())
}
