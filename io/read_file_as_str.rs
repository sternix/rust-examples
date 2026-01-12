// bir dosyayı string olarak okumak

use std::{env, fs};

fn main() {
    let filename = "abc.txt";

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// yada fonksiyon olarak

/*
fn read_to_string() -> Result<String,std::io::Error> {
    let mut file = File::open("page.html")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}
*/
