use std::thread;
use std::time;

fn main() {
    // 5 saniye bekle
    thread::sleep(time::Duration::from_secs(5));
}
