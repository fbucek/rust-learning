use std::io;
use std::thread::spawn;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug)]
struct Amazing {
    stuff: Arc<Mutex<Vec<u32>>>,
    sender: Sender<u32>,
}

fn run(a: &Amazing, receiver: Receiver<u32>) -> io::Result<()> {
    let stuff = a.stuff.clone();
    spawn(move || {
        for i in receiver {
            let mut stuff = stuff.lock().unwrap();
            stuff.push(i)
        }
    });

    Ok(())
}

fn main() {
    let (sender, receiver) = channel();
    let amazing = Amazing {
        stuff: Arc::new(Mutex::new(vec![])),
        sender,
    };

    run(&amazing, receiver).unwrap();

    let sender = &amazing.sender;
    sender.send(3).unwrap();
    sender.send(9).unwrap();
    sender.send(2).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{:?}", amazing);
}