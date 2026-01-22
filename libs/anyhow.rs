/*

[dependencies]
anyhow = "1.0"

*/

use anyhow::{Context, Result, bail};

// eğer tüm fonksiyonlar doğru çalışırsa $? = 0, bir tanesi hata verirse $? = 1
fn main() -> Result<()> {
    println!("{}", read_file()?);

    load_config()?;

    check_age(-12)?;

    Ok(())
}

fn read_file() -> Result<String> {
    let content = std::fs::read_to_string("data.txt")?;
    Ok(content)
}

fn load_config() -> Result<()> {
    let config = std::fs::read_to_string("config.toml").context("config.toml okunamadı")?;

    println!("{config}");
    Ok(())
}

fn check_age(age: i32) -> Result<()> {
    if age < 0 {
        bail!("Yaş negatif olamaz: {}", age);
    }
    Ok(())
}

