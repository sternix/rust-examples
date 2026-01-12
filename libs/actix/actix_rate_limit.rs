/*
https://crates.io/crates/actix-governor
https://medium.com/@iamfaizalkhn/putting-a-rate-limiter-in-actix-web-server-15919498dc84

cargo add actix-governor
cargo add actix_web
*/

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer, Responder, web};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Allow bursts with up to five requests per IP address
    // and replenishes one element every two seconds
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            // Enable Governor middleware
            .wrap(Governor::new(&governor_conf))
            // Route hello world service
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
