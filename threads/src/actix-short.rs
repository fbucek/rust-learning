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


pub struct Control {
    pub runner: Arc<Runner>,
}

pub struct Runner {
    pub stop: AtomicBool,
    pub running: AtomicBool,
    pub sender: Mutex<mpsc::Sender<RunnerEvent>>,
}

impl Control {
    fn run(&self) -> Result<String, &'static str> {
        if self.runner.running.load(Ordering::Relaxed) {
            Err("Control: Runner already running")
        } else {
            self.runner.stop.swap(false, Ordering::Relaxed);
            self.runner.running.swap(true, Ordering::Relaxed);
            let runner = self.runner.clone();
            thread::spawn(move || {
                loop {
                    if runner.stop.load(Ordering::Relaxed) {
                        println!("Control: Runner stop");
                        runner.running.swap(false, Ordering::Relaxed);
                        break;
                    }
                    thread::sleep(Duration::from_millis(1000));
                    println!("Runner: ping");
                }
            });
            Ok(String::from("Control: Runner started"))
        }
    }
    fn stop(&self) -> Result<&'static str, &'static str> {
        self.runner.stop.swap(true, Ordering::Relaxed);
        Ok("Control: Sucessfull stop")
    }
}

impl Runner {
    fn run(runner: Arc<Runner>, code: String) -> Result<String, &'static str> {
        if runner.running.load(Ordering::Relaxed) {
            Err("Already running")
        } else {
            runner.running.swap(true, Ordering::Relaxed);
            thread::spawn(move || {
                loop {
                    if runner.stop.load(Ordering::Relaxed) {
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
    data: web::Data<Arc<Control>>,
) -> impl Responder {
    match data.stop() {
        Ok(_) => println!("Data succesfully stopped"),
        Err(err) => println!("Problemt stopping thread {:?}", err),
    }
    // Stopping data by using shared struct Runner
    // data.stop.swap(true, Ordering::Relaxed);
    // //
    // let sender = data.sender.lock().expect("Not possible to lock sender");
    // if let Err(err) = sender.send(RunnerEvent::Run(String::from("echo commandxxx"))) { println!("Not possible to send end to sender: {:?}",err) }
    "Sending 'end' to stop thread".to_string()
}

fn start(
    data: web::Data<Arc<Runner>>,
    control: web::Data<Arc<Control>>,
) -> impl Responder {
    println!("try to start runner");
    if let Err(err) = control.run() {
        println!("Control: Not possible to start runner");
    }
    //let runner: Arc<Runner> = &data.clone();
    if let Err(err) = Runner::run(data.get_ref().clone(), String::from("echo Fine")) {
        println!("not possible to start thread {:?}", err);
    }

    "sending start to runner".to_string()
}


fn main() -> std::io::Result<()> {
    // Chanel to communicate between threads
    let (sender, receiver) = mpsc::channel();

    let data = Arc::new(Runner{
        stop: AtomicBool::new(false),
        running: AtomicBool::new(false),
        sender: Mutex::new(sender.clone()),
    });

    let control = Arc::new(Control{
        runner: data.clone(),
    });


    let data1 = data.clone();
    let data2 = data.clone();

    let handler = thread::spawn(move || {
        loop {
            if data1.stop.load(Ordering::Relaxed) {
                println!("Stopper endding thread");
                break; // ends the loop

            }
            thread::sleep(Duration::from_millis(1000));
            println!("Every seconds print");
        }
    });

    println!("http://localhost:8091/32/filip/index.html");

    let res = HttpServer::new( 
        move || App::new().service(
            web::resource("/{id}/{name}/index.html").to(index))
            .service(web::resource("/stop").to(stop))
            .service(web::resource("/start").to(start))
            .data(data2.clone())
            .data(control.clone())
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