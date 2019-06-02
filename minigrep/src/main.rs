use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("query: {} filename:{}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Not possible to read file");

    println!("Text {}", contents.trim());
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
