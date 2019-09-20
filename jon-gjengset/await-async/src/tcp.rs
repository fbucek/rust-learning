#![allow(unused)]
use std::net::{SocketAddr, TcpStream};

fn main() {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([216, 58, 201, 78], 80)),
    ];
    if let Ok(stream) = TcpStream::connect(&addrs[..]) {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}
