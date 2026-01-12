// axtix github sitesindeki örneğin redisten cookie'ye çevrilmiş hali
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_web::{App, Error, HttpRequest, HttpServer, Responder, cookie::Key, web};

async fn index(_req: HttpRequest, session: Session) -> Result<impl Responder, Error> {
    //    println!("{_req:?}");

    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {count}");
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok("Welcome!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let signing_key = Key::generate();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                signing_key.clone(),
            ))
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
