use actix_web::{App, HttpServer, Responder, web};
use tokio::process::Command;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().route("/", web::get().to(index)))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

async fn index() -> impl Responder {
    let output = Command::new("ls")
        .args(["-al", "/"])
        .output()
        .await
        .expect("Komut çalıştırılırken hata oldu");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).into_owned()
    } else {
        String::from_utf8_lossy(&output.stderr).into_owned()
    }
}
