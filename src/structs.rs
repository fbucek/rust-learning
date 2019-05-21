pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

impl Person {
    pub fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}
