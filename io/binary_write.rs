use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("binary_write.bin")?;
    let x = &mut[0];
    for i in 0..255 {
        x[0] = i;
        file.write(x)?;
    }
    Ok(())
}