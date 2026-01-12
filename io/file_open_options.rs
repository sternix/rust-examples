use std::fs::OpenOptions;
use std::io::Write;

// dosyaların mevcut olması gerekiyor

fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .open("dosya.txt")
        .expect("dosya açılamadı");
    file.write_all("Merhaba Dünya".as_bytes())
        .expect("Dosya'ya yazılamadı");

    println!("Dosyaya başarı ile yazıldı");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("dosya.txt")
        .unwrap();

    file.write_all(b"eklenecek ").expect("Dosya yazma hatası");
    // or
    write!(file, "eklenecek").expect("Dosya yazma hatası");
}
