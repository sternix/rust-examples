/*
[dependencies]
actix-web = "4.0.0-beta.8"
deadpool-postgres = "0.9"
tokio-postgres = "0.7"
bytes="1"
askama = {version="0.10", default-features = false, features = ["mime", "mime_guess"]}
*/

mod askama_actix;
// bu dosya aynı dizindeki askama_actix.rs

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use askama_actix::Template;
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

#[derive(Template)]
#[template(path = "count.html")]
struct CountTemplate {
    count: i64,
}

#[get("/")]
async fn count(pool: web::Data<deadpool_postgres::Pool>) -> String {
    let client = pool.get().await.unwrap();
    let res: i64 = client
        .query_one("SELECT COUNT(*) FROM tbltest", &[])
        .await
        .unwrap()
        .get(0);

    let hello = CountTemplate { count: res };
    hello.render().unwrap()
}

struct User {
    id: i32,
    adi: String,
    soyadi: String,
}

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {
    users: Vec<User>,
}

#[get("/users")]
async fn users(pool: web::Data<deadpool_postgres::Pool>) -> impl Responder {
    let client = pool.get().await.unwrap();
    let rows = client.query("SELECT * FROM tbltest", &[]).await.unwrap();

    let mut users = vec![];
    for row in &rows {
        let user = User {
            id: row.get("id"),
            adi: row.get("adi"),
            soyadi: row.get("soyadi"),
        };

        users.push(user);
    }

    let user_tmpl = UsersTemplate { users: users };
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(user_tmpl.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(count)
            .service(users)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

/* templates/user.html
<h1>Users</h1>
<table>
    <tr>
        <td>Id</td>
        <td>Adı</td>
        <td>Soyadı</td>
    </tr>
    {% for user in users %}
    <tr>
        <td>{{ user.id }}</td>
        <td>{{ user.adi }}</td>
        <td>{{ user.soyadi }}</td>
    </tr>
    {% endfor %}
</table>

*/

/* templates/count.html
Count of rows is {{ count }}
*/
