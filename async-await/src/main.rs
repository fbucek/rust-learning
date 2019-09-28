use std::io::prelude::*;
use tokio::net::TcpStream;
use std::net::SocketAddr;

use std::sync::{Arc, Mutex};

// @see https://www.youtube.com/watch?v=9_3krAQtD2k&t=11625s


// IMLEMENT THIS EXAMPLE
// @see https://docs.rs/crate/tokio/0.2.0-alpha.5/source/examples/tinydb.rs


struct Check {
    port: String,
    text: Mutex<String>,
    
}

impl Check {
    pub fn arc_new<S>(port: S) -> Arc<Self> where S: Into<String> {
    // pub fn new(port: str) -> Self {
        Arc::new(Check { 
            port: port.into(),
            text: Mutex::new(String::new()),
        })
    }

    pub async fn check_member(self: &Self, host: &str) -> Result<(), String> {
        //println!("Will check {}:{}", host, &self.port);

        let addr = match format!("{}:{}", host, &self.port).parse::<SocketAddr>() {
            Ok(address) => address,
            Err(_) => { return Err("Not possible to parse address".to_string()) },
        };

        println!("Will check url: {}", &addr);
        tokio::net::TcpStream::connect(&addr).await.map_err(|err| {
            println!("Not possible to connect: {:?}", err);
            {
                let mut text = self.text.lock().unwrap();
                *text = format!("Not possible to connect to: {} - error: {:?}", &addr, err);
               // println!("Text is: {}", &text);
            }
        });
        // if res.is_err() {
            
        //     //return Err(format!("not possible to connect to: {}", &addr).to_string());
        // }
        Ok(())
    } 
}


// @see https://rust-lang.github.io/async-book/print.html
fn main () -> Result<(), Box<dyn std::error::Error>> {
    //////////////////////////////
    // MacOS tcp timeout
    // Default timeout value: 60s
    //sysctl net.inet.tcp.keepinit: 75000
    // sudo sysctl net.inet.tcp.keepinit=2000

    //////////////////////////////
    // Linux
    // @See https://stackoverflow.com/a/15485241/1917249
    // @see http://willbryant.net/overriding_the_default_linux_kernel_20_second_tcp_socket_connect_timeout
    // sysctl net.ipv4.tcp_syn_retries # default 6
    // sudo sysctl net.ipv4.tcp_syn_retries=1

    // Measure time
    let now = std::time::SystemTime::now();

    // Prepare port check    
    let mut check_vec = vec![]; // Check::arc_new("23"), Check::arc_new("4545") ];
    for n in 1..800 {
        check_vec.push(Check::arc_new(n.to_string()));
    }
    println!("Array count: {}", check_vec.len());


    // let rt = tokio::runtime::Runtime::new().unwrap();
    let rt = tokio::runtime::Builder::new().core_threads(1).build().unwrap();

    for check in &check_vec {
        // Have to clone -> not possible to use sharing struct without Arc
        let clone = check.clone();
        rt.spawn(async move {
            println!("spawning");
            clone.check_member("192.168.1.2").await;
        });
    }

    rt.shutdown_on_idle();
    


    /*
    // Try to asynchonously check connected ports>
    // @see 
    for port in ports {
        let addr = format!("192.168.1.2:{}", port).parse::<SocketAddr>()?;
        println!("Will check url: {}", &addr);

        rt.spawn(async move {
            let res = tokio::net::TcpStream::connect(&addr).await;
            println!("Checked port: {}", &addr);
            if res.is_err() {
                println!("Not possible to connect: {:?}", res.err());
            }
            //let stream = listener.accept().await;
        });

        println!("Addr is: {}", addr);
        // never gets here
    }
    
    */
    
    for check in &check_vec {
        let clone = check.clone();
        let lock = clone.text.lock().unwrap();
        println!("Result: {}", &lock);
    }

    let elapsed = now.elapsed()?;
    println!("Elapsed time: {}", elapsed.as_millis());
    
    Ok(())
}



    // This code is not prepared for compilation
    // Jon writes mostly pseudo code which does not compile

    /*
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
    */
//}
