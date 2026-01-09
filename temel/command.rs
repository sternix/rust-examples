use std::io::{self, Write};
use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .args(["-al", "/"])
        .output()
        .expect("Komut çalıştırılırken hata oldu");

    if output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
    } else {
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
