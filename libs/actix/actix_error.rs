/*
[dependencies]
actix-web = "4.0.0-beta.8"
tokio = { version = "1", features = ["full"] }
derive_more = "0.99"
rand = "0.8"
env_logger = "0.9"
*/
use actix_web::{App, Error, HttpResponse, HttpServer, ResponseError, web};
use derive_more::Display;
use rand::{
    Rng,
    distributions::{Distribution, Standard},
    thread_rng,
};

#[derive(Debug, Display)]
enum CustomError {
    #[display(fmt = "Custom Error 1")]
    CustomOne,
    #[display(fmt = "Custom Error 2")]
    CustomTwo,
    #[display(fmt = "Custom Error 3")]
    CustomThree,
    #[display(fmt = "Custom Error 4")]
    CustomFour,
}

impl Distribution<CustomError> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CustomError {
        match rng.gen_range(0..4) {
            0 => CustomError::CustomOne,
            1 => CustomError::CustomTwo,
            2 => CustomError::CustomThree,
            _ => CustomError::CustomFour,
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomError::CustomOne => {
                println!("do some stuff related to CustomOne error");
                HttpResponse::Forbidden().finish()
            }

            CustomError::CustomTwo => {
                println!("do some stuff related to CustomTwo error");
                HttpResponse::Unauthorized().finish()
            }

            CustomError::CustomThree => {
                println!("do some stuff related to CustomThree error");
                HttpResponse::InternalServerError().finish()
            }

            _ => {
                println!("do some stuff related to CustomFour error");
                HttpResponse::BadRequest().finish()
            }
        }
    }
}

async fn do_something_random() -> Result<(), CustomError> {
    let mut rng = thread_rng();

    if rng.gen_bool(2.0 / 10.0) {
        Ok(())
    } else {
        Err(rand::random::<CustomError>())
    }
}

async fn do_something() -> Result<HttpResponse, Error> {
    do_something_random().await?;

    Ok(HttpResponse::Ok().body("Nothing interesting happened. Try again."))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new().service(web::resource("/").route(web::get().to(do_something)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
