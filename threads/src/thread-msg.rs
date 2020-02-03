use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//use threadlib;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let sender1 = sender.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("how"),
            String::from("are"),
            String::from("you"),
            String::from("?"),
        ];

        for val in vals {
            sender1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in receiver {
        println!("Got: {}", received);
    }
}
