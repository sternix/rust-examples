use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn main() {
    let host = "0.0.0.0:8787";

    let listener = TcpListener::bind(host).expect("Could not start listener");
    println!("Server is running on {host}");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Could not iterate over stream");
        request(&mut stream);
    }
}

fn request(stream: &mut TcpStream) {
    let mut buffer = [0; 4096];

    stream.read(&mut buffer).expect("Error reading stream");
    let req = String::from_utf8_lossy(&buffer);
    println!("{req}");
}
