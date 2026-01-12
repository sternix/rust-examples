/*
[dependencies]
actix-web = "4.0.0-beta.8"
deadpool-postgres = "0.9"
tokio-postgres = "0.7"
*/

use actix_web::{App, HttpServer, get, web};
use deadpool_postgres::{Config, ManagerConfig, Pool, PoolConfig, RecyclingMethod};
use tokio_postgres::NoTls;

async fn get_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.dbname = Some("test".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5433);
    cfg.user = Some("test".to_string());
    //    cfg.password = Some("secret".to_string());
    cfg.pool = Some(PoolConfig::new(50)); // max_pool_size
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    cfg.create_pool(NoTls).unwrap()
}

#[get("/")]
async fn index(pool: web::Data<deadpool_postgres::Pool>) -> String {
    let client = pool.get().await.unwrap();
    let res: i64 = client
        .query_one("SELECT COUNT(*) from tbltest", &[])
        .await
        .unwrap()
        .get(0);

    format!("{}", res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
