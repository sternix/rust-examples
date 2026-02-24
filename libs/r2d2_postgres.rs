/*
.env
```
DB_HOST = 127.0.0.1
DB_NAME = test
DB_PORT = 5432
DB_USER = test
DB_PASSWORD = secret
DB_POOL_SIZE = 10
```
*/

/*
Cargo.toml
```
[dependencies]
dotenv = "0.15"
postgres = {version = "0.19", features = ["with-chrono-0_4"]}
r2d2 = "0.8"
r2d2_postgres = "0.18"
```
*/

/*
foo tablosunu oluşturmak için

create table foo (bar int);
*/

use r2d2_postgres::{PostgresConnectionManager, postgres::NoTls};
use std::env;
use std::thread;

pub struct Env {
    pub host: String,
    pub dbname: String,
    pub dbport: i32,
    pub dbuser: String,
    pub dbpassword: String,
}

impl Env {
    pub fn conn_str(&self) -> String {
        format!(
            "host={} dbname={} user={} password={} port={}",
            self.host, self.dbname, self.dbuser, self.dbpassword, self.dbport
        )
    }

    pub fn parse() -> Self {
        dotenv::dotenv().ok(); // .env dosyasındaki parametreleri yüklüyor
        // böylelikle std::env::var'dan env değişkeni olarak alabiliyoruz

        Self {
            host: env::var("DB_HOST").unwrap_or("localhost".into()),
            dbname: env::var("DB_NAME").unwrap_or("test".into()),
            dbport: env::var("DB_PORT")
                .unwrap_or("5432".into())
                .parse()
                .unwrap_or(5432),
            dbuser: env::var("DB_USER").unwrap_or("postgres".into()),
            dbpassword: env::var("DB_PASSWORD").unwrap_or("secret".into()),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tipi Pool<PostgresConnectionManager<NoTls>>
    let manager = PostgresConnectionManager::new(Env::parse().conn_str().parse().unwrap(), NoTls);
    let pool = r2d2::Pool::new(manager)?;
    let mut handles = vec![];

    for i in 0..1000i32 {
        let pool = pool.clone();
        let handle = thread::spawn(move || {
            let mut client = pool.get().unwrap();
            client
                .execute("INSERT INTO foo (bar) VALUES ($1)", &[&i])
                .unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

