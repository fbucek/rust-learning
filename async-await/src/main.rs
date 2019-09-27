use std::io::prelude::*;
use tokio::net::TcpStream;
use std::net::SocketAddr;

// @see https://www.youtube.com/watch?v=9_3krAQtD2k&t=11625s


struct Check {
    port: String,
    text: String,
    
}

impl Check {
    pub fn new<S>(port: S) -> Self where S: Into<String> {
    // pub fn new(port: str) -> Self {
        Check { 
            port: port.into(),
            text: String::new(),
        }
    }

    pub async fn check(host: &str, port: &str) -> Result<(), String> {
        println!("Will check {}{}", host, port);

        let addr = match format!("{}:{}", host, port).parse::<SocketAddr>() {
            Ok(address) => address,
            Err(err) => { return Err("Not possible to parse address".to_string()) },
        };

        println!("Will check url: {}", &addr);
        let res = tokio::net::TcpStream::connect(&addr).await;
        if res.is_err() {
            println!("Not possible to connect: {:?}", res.err());
            return Err(format!("not possible to connect to: {}", &addr).to_string());
        }
        Ok(())
    } 

    pub async fn check_member(self: &mut Self, host: &str) -> Result<(), String> {
        println!("Will check {}{}", host, &self.port);

        let addr = match format!("{}:{}", host, &self.port).parse::<SocketAddr>() {
            Ok(address) => address,
            Err(err) => { return Err("Not possible to parse address".to_string()) },
        };

        println!("Will check url: {}", &addr);
        let res = tokio::net::TcpStream::connect(&addr).await;
        if res.is_err() {
            println!("Not possible to connect: {:?}", res.err());
            self.text = "Not possible to connect".to_string();
            println!("Text is: {}", &self.text);
            return Err(format!("not possible to connect to: {}", &addr).to_string());
        }
        Ok(())
    } 
}


// @see https://rust-lang.github.io/async-book/print.html
fn main () -> Result<(), Box<dyn std::error::Error>> {

    

    let mut check_vec = vec![ Check::new("23"), Check::new("4545") ];

    for n in 1..1000 {
        check_vec.push(Check::new(n.to_string()));
    }


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

    let rt = tokio::runtime::Runtime::new().unwrap();

    // 1min 15sec
    //let ports = vec![ "22", "22", "22", "22", "22", "22", "22", "22", "22", "32", "545", "332", "22", "22", "22", "22", "22", "22", "22", "22", "22", "32", "545", "332", "22", "22", "22", "22", "22", "22", "22", "22", "22", "32", "545", "332", "22", "22", "22", "22", "22", "22", "22", "22", "22", "32", "545", "332", "22", "22", "22", "22", "22", "22", "22", "22", "22", "32", "545", "332", "22", "22", "22", "22", "22", "22", "22", "22", "22", "32", "545", "332", "22", "22", "22", "22", "22", "22", "22", "22", "22", "32", "545", "332", "22", "22", "22", "22", "22", "22", "22", "22", "22", "32", "545", "332", "2323" ];
    let ports = vec![ "22" ];

    for mut check in check_vec {
        //let port = check.port.clone();
        rt.spawn(async move {
            check.check_member("192.168.1.2").await;
            //Check::check("192.168.1.2", &port).await;
        });
        //println!("{}",check.port);
    }


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
        // never gets here
    }
    
    rt.shutdown_on_idle();
    
    
    // for check in &check_vec {
    //     println!("Result: {}", check.text);
    // }


    
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
