/*

[dependencies]
tokio-postgres = "0.7"
tokio = {version = "1", features = ["full"]}

*/

use tokio_postgres::{Error, NoTls};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 dbname=test user=user password=secret port=5433",
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    // bu olmazsa düzgün çalışmıyor
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    Ok(())
}
