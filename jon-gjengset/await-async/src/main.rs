use std::io::prelude::*;
use std::net::TcpStream;

fn main () {
    let x = TcpStream::connect("127.0.0.1").unwrap();
    let y = TcpStream::connect("127.0.0.1").unwrap();
    x.write("info");
    y.write("info");

    assert_eq!(x.read().unwrap(), "response");
    assert_eq!(y.read().unwrap(), "repsonse");

    let fut_x = TcpStream::connect("127.0.0.1")
        .and_then(|c| c.write("info"))
        .and_then(|c| c.read())
        .and_then(|(c, b)|  b == "response");
    println!("{:?}", fut_x);

    let a: Executor;
    let x = a.run(fut_x);
    let y = a.run(fut_y);

    a.spawn(fut_x.and_then(|eq| assert!(eq)));
    a.spawn(fut_y.and_then(|eq| assert!(eq)));
    a.block_on_all();
}
