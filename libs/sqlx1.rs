/*
cargo add sqlx --features runtime-tokio-native-tls,postgres,uuid,chrono,migrate
cargo add tokio --features full
*/

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://test_user:secret@127.0.0.1/test")
        .await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(28_i64)
        .fetch_one(&pool)
        .await?;

    println!("Row: {:?}", row.0);

    Ok(())
}
