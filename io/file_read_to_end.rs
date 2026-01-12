use std::io::Read;

fn read_a_file() -> std::io::Result<Vec<u8>> {
    let mut file = File::open("xyz.rs")?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}

fn main() {
    for l in read_a_file().unwrap() {
        println!("{}", l);
    }
}
