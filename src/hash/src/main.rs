use sha2::{Sha256, Digest};

fn main() {
	let s = "Merhaba Dünya";
	println!("{}'ın SHA256 hash'ı: {}",s,get_sha256_hash(s));

	println!("sha256 crate'i ile");
	println!("{}'ın SHA256 hash'ı: {}",s,get_sha256_hash_2(s));
}

fn get_sha256_hash(s: &str) -> String {
	let hash = Sha256::digest(s.as_bytes());
	base16ct::lower::encode_string(&hash)
}


use sha256::digest;

fn get_sha256_hash_2(s: &str) -> String {
	digest(s)
}
