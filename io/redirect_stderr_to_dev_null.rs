// yada /dev/null'a yönlendirmek

use std::fs::File;
use std::io::Error;
use std::process::{Command, Stdio};

fn main() -> Result<(), Error> {
    let outputs = File::open("/dev/null")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

fn _ls() -> Result<(), Error> {
    let errors = File::open("/dev/null")?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}
