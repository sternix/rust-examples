use postgres::{Client, NoTls};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::configure()
        .host(std::env::var("HOST")?.as_str())
        .password(std::env::var("DBNAME")?.as_str())
        .port(5432)
        .user(std::env::var("USER")?.as_str())
        .password(std::env::var("PASSWORD")?.as_str())
        .connect(NoTls)?;

    let res = client.query("SELECT version()", &[])?;

	// tip vererek
    let ver: String = res[0].get(0);
    println!("{}", ver);

	// yada

	//turbofish ile
    let ver = res[0].get::<_,String>(0);
    println!("{}", ver);

    Ok(())
}

//  HOST=192.168.1.1 DBNAME=test USER=postgres PASSWORD='secret' cargo run

