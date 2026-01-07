use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};

// 14 haneli HES kodundan - ve boşluk varsa çıkarıp büyük harf yapan program

fn main() {
    let file = File::open("hes.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut wr = File::create("hes_sonuc.txt").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();

        let mut nl = line.replace("-", "").replace(" ", "").to_uppercase();
        if nl.len() != 10 {
            nl = format!("HATA:{}\n", nl);
        } else {
            nl = format!("{}\n", nl);
        }

        wr.write_all(nl.as_bytes()).unwrap();
    }
}
