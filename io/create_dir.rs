static STR: &str = "Test";

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // mevcut dizinde xyz klasörü oluşturuyoruz
    let mut path = Path::new(".").join("xyz");
    fs::create_dir(&path)?;

    // o dizinin içerisinde abc.txt dosyasına STR'nin içeriğini yazıyoruz
    path = path.join("abc.txt");

    let mut file = File::create(&path)?;

    file.write_all(STR.as_bytes())?;

    Ok(())
}
