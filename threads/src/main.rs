use std::thread;
use std::time::Duration;
//use threadlib;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Number {} from thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    
    for i in 1..5 {
        println!("Nubmer {}, from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

}
