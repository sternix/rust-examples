use std::fs::File;
use std::io::{self, ErrorKind, Read, Write};

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; DEFAULT_BUF_SIZE];
    let mut written = 0;

    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        writer.write_all(&buf[..len])?;
        written += len as u64;
    }
}

fn main() {
    let mut r = File::open("copy_file.rs").unwrap();
    let mut w = File::create("xyz.txt").unwrap();

    match copy(&mut r, &mut w) {
        Ok(n) => println!("{} bytes written", n),
        Err(e) => println!("failed with {} ", e),
    }
}
