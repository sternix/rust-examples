use std::env;
use std::fs::File;
use std::io::Read; // read_to_string için

fn main() {
    let first = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&first).expect("can't open the file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    println!("file had {} bytes", text.len());
}
