// use testlib;

fn main() {
    // We will iterate through the references to the element returned by
    // env::vars();
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }
    let name = std::env::var("CARGO_PKG_NAME");
    println!("name: {}", name.unwrap_or_else(|_| "test".to_string()));
}
