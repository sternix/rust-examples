// http auth

/*

[dependencies]
reqwest = { version = "0.11", features = ["blocking"]}

 */

use reqwest::Error;
use reqwest::blocking::Client;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user_name = "xuser".to_string();
    let password: Option<String> = Some("secret".to_string());

    let response = client
        .get("https://example.com")
        .basic_auth(user_name, password)
        .send();

    println!("{:?}", response);

    Ok(())
}
