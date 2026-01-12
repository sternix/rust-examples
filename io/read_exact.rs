// https://fasterthanli.me/series/reading-files-the-hard-way/part-3

//use hex_slice::AsHex;
use std::{fs::OpenOptions, io::Read};

fn main() -> Result<(), std::io::Error> {
    // open our ext4 partition, READ-ONLY.
    let mut file = OpenOptions::new().read(true).open("/dev/sda3")?;

    // allocate a 128-byte buffer
    let mut buf = vec![0u8; 128];

    // read the first 128 bytes of the file
    file.read_exact(&mut buf)?;

    // print it as hexadecimal
    // println!("{:x}", buf.as_hex());
    println!("{:x}", buf);

    Ok(())
}
