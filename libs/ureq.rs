/*

[dependencies]
ureq = "3"

*/

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let body: String = ureq::get("https://jsonplaceholder.typicode.com/posts")
        .call()?
        .body_mut()
        .read_to_string()?;

    println!("{body}");

    Ok(())
}

