//! Example how to guard variable
//! @see https://users.rust-lang.org/t/solved-help-with-shared-data-and-mutexes/5323

use actix_web::{web, App, HttpServer, Responder};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use std::process::Command;

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
}

/// Runner -> have to add some documentations
impl Runner {
    fn start(&self) {
        loop {
            if self.stop.load(Ordering::Relaxed) {
                println!("Control: Runner stop");
                self.running.swap(false, Ordering::Relaxed);
                break;
            }
            thread::sleep(Duration::from_millis(1000));

            let cmd = Command::new("git")
                .arg("--version")
                .output()
                .expect("Not possible to run comnad");

            println!("Process status: {}", cmd.status);
            let out = String::from_utf8_lossy(&cmd.stdout);

            println!("Runner: output: {}", out);
        }
    }
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
                runner.start();
            });
            Ok(String::from("Control: Runner started"))
        }
    }
    fn stop(&self) -> Result<&'static str, &'static str> {
        self.runner.stop.swap(true, Ordering::Relaxed);
        Ok("Control: Sucessfull stop")
    }
}

async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    println!("OK");
    format!("Hello {}! id:{}", info.1, info.0)
}

async fn stop(data: web::Data<Arc<Control>>) -> impl Responder {
    if let Err(err) = data.stop() {
        println!("Control: Not possible to stop runner: {:?}", err)
    }
    "Sending 'end' to stop thread".to_string()
}

async fn start(control: web::Data<Arc<Control>>) -> impl Responder {
    if let Err(err) = control.run() {
        println!("Control: Not possible to start runner: {:?}", err)
    }
    "Control: trying to start runner".to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let runner = Arc::new(Runner {
        stop: AtomicBool::new(false),
        running: AtomicBool::new(false),
    });

    let control = Arc::new(Control {
        runner: runner.clone(),
    });

    let control1 = control.clone();
    println!("http://localhost:8091/32/filip/index.html");

    let server_future = HttpServer::new(move || {
        App::new()
            .service(web::resource("/{id}/{name}/index.html").to(index))
            .service(web::resource("/stop").to(stop))
            .service(web::resource("/start").to(start))
            .data(control1.clone())
    })
    .bind("127.0.0.1:8091")
    .expect("Not possible to bind to address")
    .run();

    // Have to stop runners
    control.stop().unwrap();
    server_future.await
}
