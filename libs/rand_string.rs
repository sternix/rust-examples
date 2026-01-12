/*
random string üretmek
take'teki 30 karakter'i değiştirerek istediğimiz uzunlukta bir string elde edebiliriz

https://github.com/metaplex-foundation/digital-asset-rpc-infrastructure/blob/main/nft_ingester/src/main.rs#L107-L113


[dependencies]
rand = "0.8"

*/

use rand::{Rng, distributions::Alphanumeric, thread_rng};

fn rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

fn main() {
    println!("{}", rand_string());
}
