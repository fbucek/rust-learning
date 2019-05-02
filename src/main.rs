mod print;
mod types;
mod structs;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let first_argument = args[1].clone();

    println!("First argument is: {}", first_argument);

    println!("😀Hello, world!");

    print::run();

    types::run();

    let p = structs::Person::new("John", "Doe");
    println!("Name is: {} and surname is: {}", p.first_name, p.last_name);
}
