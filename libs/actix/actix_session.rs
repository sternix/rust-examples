// https://webbureaucrat.gitlab.io/articles/setting-and-reading-session-cookies-in-rust-with-actix-web/

/*
[dependencies]
# cargo add actix-session --features=cookie-session
actix-session = { version = "0.10.0", features = ["cookie-session"] }
actix-web = "4.9.0"
*/

use actix_session::SessionMiddleware;
use actix_session::config::{BrowserSession, CookieContentSecurity};
use actix_session::storage::CookieSessionStore;
use actix_web;
use actix_web::cookie::{Key, SameSite};

#[actix_web::get("/")]
async fn index() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("Hello, world")
}

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_name(String::from("my-kata-cookie")) // arbitrary name
        .cookie_secure(true) // https only
        .session_lifecycle(BrowserSession::default()) // expire at end of session
        .cookie_same_site(SameSite::Strict)
        .cookie_content_security(CookieContentSecurity::Private) // encrypt
        .cookie_http_only(true) // disallow scripts from reading
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(session_middleware())
            .service(index)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
