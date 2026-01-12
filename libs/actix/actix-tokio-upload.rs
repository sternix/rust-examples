/*
[dependencies]
tokio = {version="1", features= ["fs"]}
actix-web = "4.0.0-beta.8"
actix-multipart = "0.4.0-beta.5"
futures = "0.3"
sanitize-filename = "0.3"
*/

use actix_multipart::Multipart;
use actix_web::{App, Error, HttpResponse, HttpServer, http, middleware, web};
use futures::{StreamExt, TryStreamExt};
use tokio::io::AsyncWriteExt;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
        let mut f = tokio::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }

    Ok(HttpResponse::Found()
        .append_header((http::header::LOCATION, "/"))
        .finish())
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    tokio::fs::create_dir_all("./tmp").await?;

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(save_file)),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
