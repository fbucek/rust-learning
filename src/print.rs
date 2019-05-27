/// Print specific things
pub fn run() {
    // Print to console
    println!("Hello number 34");

    let years = 34i64;
    let mut name = "John";

    println!(
        "{name} likes to play baseball for ({year:b}) binary",
        name = name,
        year = years
    );
    name = "Mark";
    println!(
        "{name} likes to play baseball for ({year:x}) hexadecimal",
        name = name,
        year = years
    );
}
