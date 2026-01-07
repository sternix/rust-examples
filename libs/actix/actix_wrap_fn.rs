/*
[dependencies]
actix-web = "4.0.0-beta.8"
futures-util = "0.3"
*/

use actix_web::dev::Service;
use actix_web::{App, HttpServer, Responder, web};
use futures_util::FutureExt;

async fn index() -> impl Responder {
    println!("index çağrıldı");
    "Hello from index"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("Hi from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    println!("Hi from response");
                    res
                })
            })
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
