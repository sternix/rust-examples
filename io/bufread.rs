use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let file = File::open("/etc/rc.conf")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    // uzun hali, fakat Err geldiğinde hata vermiyor
    /*
    for line in reader.lines() {
        if let Ok(l) = line {
            println!("{}", l);
        }
    }
    */

    Ok(())
}
