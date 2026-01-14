use std::io::prelude::*;

fn main() {
    let stdin = std::io::stdin();
    println!("{}", stdin.lock().lines().count());
}

// programı çalıştırdıktan sonra 3,5 satır gir sonra CTRL + D'ye bas
// kaç satır girildiğini yazıyor
