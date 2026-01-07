use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;

/*
/var/log dizinini log.tar.gz dosyasına mevcut dizinde sıkıştırıyor,
tar dosyası içinde log klasörüne
*/

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("logs.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("log", "/var/log/")?;

    Ok(())
}
