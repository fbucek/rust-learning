use looplib;

fn main() {
    println!("Hello, world!");

    let counter = looplib::Counter::new();

    let vec: Vec<u32> = counter.collect();

    println!("vec is: {:?}", &vec);
}


