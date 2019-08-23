//! Example how to guard variable
//! @see https://users.rust-lang.org/t/solved-help-with-shared-data-and-mutexes/5323


use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use actix_web::{web, App, HttpServer, Responder};

pub enum RunnerEvent {
    Run(String),
    Message(String),
}

pub struct Runner {
    pub stop: AtomicBool,
    pub running: AtomicBool,
    pub sender: Mutex<mpsc::Sender<RunnerEvent>>,
}

impl Runner {
    fn run(&mut self, code: String) -> Result<String, &'static str> {
        if self.running.load(Ordering::Release) {
            Err("Already running")
        } else {
            self.running.swap(true, Ordering::Release);
            thread::spawn(move || {
                loop {
                    if self.stop.load(Ordering::Relaxed) {
                        println!("Runner: stop");
                        break; // ends the loop

                    }
                    thread::sleep(Duration::from_millis(1000));
                    println!("Runner: ping");
                }
            });
            Ok(String::from("Started"))
        }
    }
}


fn index(
    info: web::Path<(u32, String)>
) -> impl Responder {
    println!("OK");
    format!("Hello {}! id:{}", info.1, info.0)
}

fn stop(
    data: web::Data<Arc<Runner>>,
) -> impl Responder {
    // Stopping data by using shared struct Runner
    data.stop.swap(true, Ordering::Relaxed);
    //
    let sender = data.sender.lock().expect("Not possible to lock sender");
    if let Err(err) = sender.send(RunnerEvent::Run(String::from("echo commandxxx"))) { println!("Not possible to send end to sender: {:?}",err) }
    "Sending 'end' to stop thread".to_string()
}

fn start(
    data: web::Data<Arc<Runner>>,
) -> impl Responder {
    println!("try to start runner");
    &data.run(String::from("echo Fine"));
}


fn main() -> std::io::Result<()> {
    // Chanel to communicate between threads
    let (sender, receiver) = mpsc::channel();

    let data = Arc::new(Runner{
        stop: AtomicBool::new(false),
        running: AtomicBool::new(false),
        sender: Mutex::new(sender.clone()),
    });

    let data1 = data.clone();
    let data2 = data.clone();

    let handler = thread::spawn(move || {
        loop {
            if data1.stop.load(Ordering::Relaxed) {
                println!("Stopper endding thread");
                break; // ends the loop

            }
            //let message = receiver.try_recv();
            // if message.is_ok() && message.unwrap() == RunnerEvent {
            //     println!("Message 'end' ending thread");
            //     break; // ends the loop
            // }
            thread::sleep(Duration::from_millis(1000));
            println!("Every seconds print");
        }
        // let data = data.clone();
        // data.value.
        // let vals = vec![
        //     String::from("hi"),
        //     String::from("from"),
        //     String::from("the"),
        //     String::from("thread"),
        // ];

        // for val in vals {
        //     tx.send(val).unwrap();
        // }
    });



    println!("http://localhost:8091/32/filip/index.html");

    let res = HttpServer::new( 
        move || App::new().service(
            web::resource("/{id}/{name}/index.html").to(index))
            .service(web::resource("/stop").to(stop))
            .service(web::resource("/start").to(start))
            .data(data2.clone())
            )
        .bind("127.0.0.1:8091")?
        .run();

    // Have to send end to application
    //if let Err(err) = sender.send(&"end") { println!("Not possible to stop thread ( probably not running ) {:?}", err ) }
    //if let
    data.stop.swap(true, Ordering::Relaxed);
    if let Err(err) = handler.join() { println!("Not possible to join thread: {:?}", err) }

    res
}