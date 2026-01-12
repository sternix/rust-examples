use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let read = File::open(r#"E:\1.xls"#);

    let write = OpenOptions::new()
        .write(true)
        .create(true)
        .open(r#"E:\2.xls"#);

    let mut reader = BufReader::new(read.unwrap());

    let mut writer = BufWriter::new(write.unwrap());

    let mut length = 1;

    while length > 0 {
        let buffer = reader.fill_buf().unwrap();

        writer.write(buffer);

        length = buffer.len();
        reader.consume(length);
    }
}
