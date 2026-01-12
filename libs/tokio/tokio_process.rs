/*

service nginx komutları ROOT olarak çalıştır

[dependencies]
actix-web = "4.0.0-beta.19"
tokio = { version = "1", features = ["rt-multi-thread","process"] }

*/

use actix_web::{App, HttpServer, Responder, http::Uri, web};
use tokio::process::Command;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/start", web::get().to(page))
            .route("/stop", web::get().to(page))
            .route("/status", web::get().to(page))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

enum SrvCommand {
    Start,
    Stop,
    Status,
}

async fn page(uri: Uri) -> impl Responder {
    match uri.path() {
        "/start" => cmd(SrvCommand::Start).await,
        "/stop" => cmd(SrvCommand::Stop).await,
        "/status" => cmd(SrvCommand::Status).await,
        _ => "HATA".to_string(),
    }
}

async fn cmd(cmd: SrvCommand) -> String {
    let a = match cmd {
        SrvCommand::Start => "start",
        SrvCommand::Stop => "stop",
        SrvCommand::Status => "status",
    };

    let output = Command::new("service")
        .args(["nginx", a])
        .output()
        .await
        .expect("Komut çalıştırılırken hata oldu");

    String::from_utf8_lossy(&output.stdout).into_owned()
}
