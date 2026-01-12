// bir klasörü sıkıştırmak

/*

[dependencies]
flate2 = "1"
tar = "0.4"

*/

use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("logs.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);

    // /var/log dizinini
    // backup/logs dinine alıyor
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}
