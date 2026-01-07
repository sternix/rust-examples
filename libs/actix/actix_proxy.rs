/*
actix-web = "4.0.0-beta.8"
clap = "3.0.0-beta.4"
url = "2"
awc = "3.0.0-beta.7"
*/

// cargo run 0.0.0.0 8080 127.0.0.1 80
// gelen istekleri 127.0.0.1:80'de çalışan sunucuya gönderip sonucu tekrar bize gönderiyor
// kısaca proxy - vekillik yapıyor

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use awc::Client;
use clap::Arg;
use std::net::ToSocketAddrs;
use url::Url;

async fn forward(
    req: HttpRequest,
    body: web::Bytes,
    url: web::Data<Url>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    let mut new_url = url.get_ref().clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();

    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.insert_header(("x-forwarded-for", format!("{}", addr.ip())))
    } else {
        forwarded_req
    };

    let mut res = forwarded_req.send_body(body).await.unwrap(); //.map_err(Error::from)?;
    let mut client_resp = HttpResponse::build(res.status());

    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.insert_header((header_name.clone(), header_value.clone()));
    }

    Ok(client_resp.body(res.body().await?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("HTTP proxy")
        .arg(
            Arg::new("listen_addr")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("listen_port")
                .takes_value(true)
                .value_name("LISTEN PORT")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::new("forward_addr")
                .takes_value(true)
                .value_name("FWD ADDR")
                .index(3)
                .required(true),
        )
        .arg(
            Arg::new("forward_port")
                .takes_value(true)
                .value_name("FWD PORT")
                .index(4)
                .required(true),
        )
        .get_matches();

    let listen_addr = matches.value_of("listen_addr").unwrap();
    let listen_port: u16 = matches
        .value_of_t("listen_port")
        .unwrap_or_else(|e| e.exit());
    let forwarded_addr = matches.value_of("forward_addr").unwrap();
    let forwarded_port: u16 = matches
        .value_of_t("forward_port")
        .unwrap_or_else(|e| e.exit());

    let forwarded_url = Url::parse(&format!(
        "http://{}",
        (forwarded_addr, forwarded_port)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    ))
    .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Client::new()))
            .app_data(web::Data::new(forwarded_url.clone()))
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(forward))
    })
    .bind((listen_addr, listen_port))?
    .system_exit()
    .run()
    .await
}
