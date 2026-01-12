/*
[dependencies]
ssh2 = "0.9"
anyhow = "1.0"
*/

use anyhow::Error;
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

fn main() -> Result<(), Error> {
    let stream = TcpStream::connect(format!("{}:22", "192.168.1.1"))?;
    println!("Connected to the server!");
    let mut session = Session::new()?;
    session.set_tcp_stream(stream);
    session.handshake().unwrap();
    session.userauth_password("username", "secret")?;
    println!("Authenticated!");
    let mut channel = session.channel_session()?;
    channel.exec("whoami").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    let exit_status = channel.exit_status().unwrap();
    if exit_status != 0 {
        eprint!("Exited with status {}", exit_status);
    }

    Ok(())
}
