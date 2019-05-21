mod print;
mod structs;
mod types;

use std::env;
use std::process;

//#![deny(missing_docs)]
//#![deny(unused_extern_crates)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments");
        process::exit(1);
    }

    let first_argument = args[1].clone();

    println!("First argument is: {}", first_argument);

    println!("😀Hello, world!");

    print::run();

    types::run();

    let p = structs::Person::new("John", "Doe");
    println!("Name is: {} and surname is: {}", p.first_name, p.last_name);
}
