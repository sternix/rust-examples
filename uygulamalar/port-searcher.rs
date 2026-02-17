/*

A port scanner written in Rust using Clap.

https://medium.com/@carlosmarcano2704/how-to-build-a-cli-tool-to-show-ports-available-rust-61f085aef60e

https://github.com/carlosm27/port-searcher/tree/master

cargo run -- --ports 4000 3000 5000 9000 8080 9090 5174

port-searcher --ports 4000 3000 5000 9000 8080 9090 5174

*/

/*

[dependencies]
clap = { version = "4.4.6", features = ["derive"] }

*/


use std::net::TcpListener;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(num_args(0..), short, long)]
    ports: Vec<u16>,
}

fn main() {
    let args = Args::parse();
    let ports = args.ports;

    ports_available(ports.to_vec())
}

fn ports_available(ports: Vec<u16>) {
    for port in ports {
        if port_is_available(port) {
            println!("port: {} is available", port);
        } else {
            println!("Port: {} is NOT available", port)
        }
    }
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}



