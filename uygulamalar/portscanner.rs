// https://github.com/ALi3nW3rX/portscanner

/*
[dependencies]
clap = { version = "4.4.6", features = ["derive"] }
*/

// NOT: async değil thread tablanlı

use clap::Parser;
use std::collections::VecDeque;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Simple rust portscanner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ip address of target
    #[arg(short, long)]
    ip: IpAddr,

    /// Number of threads to use
    #[arg(short, long)]
    threads: u16,
}

const START_PORT: u16 = 1;
const END_PORT: u16 = 65535;

fn portscan(ip: IpAddr, port: u16, timeout_duration: Duration) -> Result<(), String> {
    let conn = SocketAddr::new(ip, port);
    match std::net::TcpStream::connect_timeout(&conn, timeout_duration) {
        Ok(_) => {
            println!("{} is open", port);
            Ok(())
        }
        Err(_) => Err(format!("{}", port)),
    }
}

fn main() {
    let args = Args::parse();
    let ip = args.ip;
    let timeout_duration = Duration::from_secs(3);

    let scanports = Arc::new(Mutex::new(
        (START_PORT..END_PORT).collect::<VecDeque<u16>>(),
    ));
    let mut handles = vec![];

    let threads = args.threads;
    for _ in 0..threads {
        let scanports = scanports.clone();
        let ip = ip.clone();
        let timeout_duration = timeout_duration.clone();
        let handle = thread::spawn(move || {
            while let Some(port) = {
                let mut ports = scanports.lock().unwrap();
                ports.pop_front()
            } {
                if let Err(_) = portscan(ip, port, timeout_duration) {
                    let mut ports = scanports.lock().unwrap();
                    ports.push_back(port);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

