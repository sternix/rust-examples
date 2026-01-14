fn main() {
    println!(
        "Dosya: {} - Satır: {} - Sütun: {}",
        file!(),
        line!(),
        column!()
    );
}
