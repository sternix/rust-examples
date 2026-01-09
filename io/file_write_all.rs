use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Hello World".as_bytes())
        .expect("write failed");
    file.write_all("\nMerhaba Dünya".as_bytes())
        .expect("write failed");
    println!("data written to file");
}
