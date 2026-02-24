
/*
.env
```
DB_HOST = 127.0.0.1
DB_NAME = test
DB_PORT = 5432
DB_USER = test
DB_PASSWORD = secret
DB_POOL_SIZE = 10
*/

/*
foo tablosunu oluşturmak için

create table foo (bar int);
*/

/*
Cargo.toml
```
[dependencies]
dotenvy = "0.15"
futures = "0.3"
tokio = {version = "1", features = ["full"]}
tokio-postgres = {version = "0.7", features = ["with-chrono-0_4","with-serde_json-1"]}
deadpool-postgres = "0.14"
```

*/

use deadpool_postgres::{Config, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime};
use futures::future;
use std::env;
use tokio_postgres::NoTls;

struct Env {
    db_host: String,
    db_name: String,
    db_port: u16,
    db_user: String,
    db_password: String,
    db_pool_size: usize,
}

impl Env {
    fn parse() -> Self {
        dotenvy::dotenv().ok();

        Self {
            db_host: env::var("DB_HOST").unwrap_or("localhost".into()),
            db_name: env::var("DB_NAME").unwrap_or("test".into()),
            db_port: env::var("DB_PORT")
                .unwrap_or("5432".into())
                .parse()
                .unwrap_or(5432),
            db_user: env::var("DB_USER").unwrap_or("postgres".into()),
            db_password: env::var("DB_PASSWORD").unwrap_or("secret".into()),
            db_pool_size: env::var("DB_POOL_SIZE")
                .unwrap_or("10".into())
                .parse()
                .unwrap_or(10),
        }
    }
}

async fn get_pool() -> Pool {
    let env = Env::parse();

    let mut cfg = Config::new();
    cfg.host = Some(env.db_host);
    cfg.dbname = Some(env.db_name);
    cfg.port = Some(env.db_port);
    cfg.user = Some(env.db_user);
    cfg.password = Some(env.db_password);

    cfg.pool = Some(PoolConfig::new(env.db_pool_size));
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool().await;
    let mut v = vec![];
    for i in 0..1_000 {
        let pool = pool.clone();
        let handle = tokio::spawn(async move {
            let client = pool.get().await.unwrap();
            client
                .execute("INSERT INTO foo (bar) VALUES ($1)", &[&i])
                .await
                .unwrap();
        });

        v.push(handle);
    }

    future::join_all(v).await;

    Ok(())
}

