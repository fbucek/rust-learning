use std::io::prelude::*;
use std::net::TcpStream;

fn main () {
    let x = TcpStream::connect("127.0.0.1").unwrap();
    let y = TcpStream::connect("127.0.0.1").unwrap();
    x.write("info");
    y.write("info");

    assert_eq!(x.read().unwrap(), "response");
    assert_eq!(y.read().unwrap(), "repsonse");

    let x = TcpStream::connect("127.0.0.1")
        .and_then(|c| c.write("info"))
        .and_then(|c| c.read())
        .and_then(|(c, b)|  b == "response");
    println!("{:?}", fut);
        


}
