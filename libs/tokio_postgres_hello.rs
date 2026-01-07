/*
[dependencies]
tokio-postgres = "0.7.6"
tokio = {version="1", features= ["macros","rt-multi-thread"]}
*/

use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 user=test dbname=test password=secret",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let rows = client.query("select $1::TEXT", &[&"hello world"]).await?;

    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    Ok(())
}
