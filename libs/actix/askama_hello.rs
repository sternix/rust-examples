/*
[dependencies]
actix-web="3"
askama = "0.10"
askama_actix = "0.11"
futures-util = "0.3"
*/

use actix_web::{App, HttpServer, Responder, get};
use askama_actix::Template;

// crate'in root dizinine templates adlı bir klasörün içinde
// hello.html adlı, içeriği
// Hello {{ name }}!
// olan bir dosya oluşturalım

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[get("/")]
async fn index() -> impl Responder {
    // istersek template'i direkt gönderebiliriz
    // content-type: text/html
    HelloTemplate { name: "Dünya" }

    // istersek render() ile sonucu String olarak gönderebiliriz
    // content-type: text/plain
    /*
    let hello = HelloTemplate { name: "Dünya" };
    hello.render().unwrap()
    */
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
