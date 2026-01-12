/*
[dependencies]
actix-web = "4.0.0-beta.8"
serde = "1"
*/

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, error, http::Method, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Index {
    id: String,
}

impl Index {
    async fn index(
        req: HttpRequest,
        form: Option<web::Form<Index>>,
    ) -> Result<HttpResponse, Error> {
        match req.method() {
            &Method::GET => Index::get().await,
            &Method::POST => {
                if form.is_some() {
                    Index::post(form.unwrap().into_inner()).await
                } else {
                    Err(error::ErrorNotAcceptable("Form eksik"))
                }
            }
            _ => Err(error::ErrorMethodNotAllowed(
                "Bu methoda izin verilmiyor!!!",
            )),
        }
    }

    async fn get() -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body("Hello from GET"))
    }

    async fn post(form: Index) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Hello from POST, id is {}", form.id)))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::to(Index::index)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
