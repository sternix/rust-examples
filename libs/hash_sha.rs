/*
cargo add base16ct sha2 sha256
*/

/*
[dependencies]
base16ct = { version = "0.2", features = ["alloc"] }
sha2 = "0.10.8"
sha256 = "1.5.0"
*/

use sha2::{Digest, Sha256};

fn main() {
    let s = "Merhaba Dünya";
    println!("{}'ın SHA256 hash'ı: {}", s, get_sha256_hash(s));

    println!("sha256 crate'i ile");
    println!("{}'ın SHA256 hash'ı: {}", s, get_sha256_hash_2(s));
}

fn get_sha256_hash(s: &str) -> String {
    let hash = Sha256::digest(s.as_bytes());
    base16ct::lower::encode_string(&hash)
}

use sha256::digest;

fn get_sha256_hash_2(s: &str) -> String {
    digest(s)
}
