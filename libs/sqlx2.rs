// https://gist.github.com/jeremychone/34d1e3daffc38eb602b1a9ab21298d10

/*
[dependencies]
sqlx = { version = "0.8.0", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono", "migrate","macros"] }
tokio = { version = "1.39.2", features = ["full"] }
*/

use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};

/*

CREATE TABLE tbltest (
    id int gererated always as identity primary key,
    adi varchar(255),
    soyadi varchar(255)
)

*/

#[derive(Debug, FromRow)]
struct TblTest {
    id: i32,
    adi: String,
    soyadi: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://test_user:secret@127.0.0.1/test")
        .await?;

    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS tbltest (
	id int generated always as identity primary key,
	adi varchar(255),
	soyadi varchar(255)
);
	"#,
    )
    .execute(&pool)
    .await?;

    let row: (i32,) = sqlx::query_as("INSERT INTO tbltest VALUES (DEFAULT,$1,$2) RETURNING id")
        .bind("test_adi")
        .bind("test_soyadi")
        .fetch_one(&pool)
        .await?;

    println!("yeni id: {}", row.0);

    let rows = sqlx::query("SELECT * FROM tbltest")
        .fetch_all(&pool)
        .await?;
    let str_result = rows
        .iter()
        .map(|r| {
            format!(
                "{}-{}-{}",
                r.get::<i32, _>("id"),
                r.get::<String, _>("adi"),
                r.get::<String, _>("soyadi")
            )
        })
        .collect::<Vec<String>>()
        .join(", ");

    println!("\n== Tüm tbltest kayıtları:\n{}", str_result);

    // manuel row oluşturuyoruz
    let select_query = sqlx::query("SELECT id,adi,soyadi FROM tbltest");
    let tests: Vec<TblTest> = select_query
        .map(|row: PgRow| TblTest {
            id: row.get("id"),
            adi: row.get("adi"),
            soyadi: row.get("soyadi"),
        })
        .fetch_all(&pool)
        .await?;

    println!("\n== Tüm tbltest kayıtları2:\n");
    for test in tests {
        println!("{}-{}-{}", test.id, test.adi, test.soyadi);
    }

    // derive ile row oluşturuluyor
    let select_query = sqlx::query_as::<_, TblTest>("SELECT * FROM tbltest");
    let tbltests: Vec<TblTest> = select_query.fetch_all(&pool).await?;
    println!("\n== Tüm tbltest kayıtları3:\n");
    for test in tbltests {
        println!("{}-{}-{}", test.id, test.adi, test.soyadi);
    }

    let tests: Vec<TblTest> = sqlx::query!("SELECT id,adi,soyadi FROM tbltest")
        .fetch_all(&pool)
        .await?
        .iter()
        .map(|row| TblTest {
            id: row.id,
            adi: row.adi.clone().unwrap(),
            soyadi: row.soyadi.clone().unwrap(),
        })
        .collect();

    println!("\n== Tüm tbltest kayıtları 4:\n");
    for test in tests {
        println!("{}-{}-{}", test.id, test.adi, test.soyadi);
    }

    Ok(())
}
