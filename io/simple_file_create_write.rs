use std::fs::File;
use std::io::Write;

fn main() {
    let mut f = File::create("hello.txt").unwrap();
    f.write(b"Hello World").unwrap();
    f.flush().unwrap();
}
