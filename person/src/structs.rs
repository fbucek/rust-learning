pub struct Person {
    /// Human must have first name
    pub first_name: String,
    /// Human must have last name
    pub last_name: String,
}

/// Defined human
impl Person {
    pub fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}
