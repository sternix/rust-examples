/*
[dependencies]
mime = "0.3"
reqwest = { version = "0.10", features = ["cookies"] }
tokio = { version = "0.2", features = ["full"] }
*/

use std::path::Path;

static UA: &str = "kattis-cli-submit";
static POSTURL: &str = "https://httpbin.org/post";
static SETCOOKIE: &str = "https://httpbin.org/cookies/set/foo/bar";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent(UA)
        .cookie_store(true)
        .build()?;
    // sets a cookie, we should be able to see this in
    // the responses, as the session object will re-send it
    client.get(SETCOOKIE).send().await?;

    // Login
    // could also use a HashMap
    let login_fields = [
        ("user", "mjpieters"),
        ("script", "true"),
        ("token", "[TOKEN]"),
    ];
    let login_response = client
        .post(POSTURL)
        .form(&login_fields)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", login_response);

    // Make a submission
    let mut form = reqwest::multipart::Form::new()
        .text("submit", "true")
        .text("submit_ctr", "2")
        .text("language", "en")
        .text("mainclass", "problem?")
        .text("problem", "problem!")
        .text("script", "true");

    // add files
    let mime_type = mime::APPLICATION_OCTET_STREAM.to_string();
    let filenames = vec!["assets/submission1.txt", "assets/submission2.txt"];
    for (i, &fname) in filenames.iter().enumerate() {
        let path = Path::new(fname);
        let sub_file_contents = std::fs::read(path)?;
        let sub_file_part = reqwest::multipart::Part::bytes(sub_file_contents)
            .file_name(path.file_name().unwrap().to_string_lossy())
            .mime_str(&mime_type)?;

        // httpbin puts all files into a JSON object under
        // the part name. Multiple parts with the same name are
        // perfectly fine but httpbin only shows _one_. So for
        // this example, we add a part number:
        let part_name = format!("sub_file[{}]", i);
        form = form.part(part_name, sub_file_part);
    }
    let submission_response = client
        .post(POSTURL)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;

    println!("Submission response:\n{}", submission_response);
    Ok(())
}
