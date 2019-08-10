use std::collections::HashMap;

fn hash_from_vectors() {
    let teams = vec![String::from("Blue"), String::from("Green")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn invalid_maps() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name was moved into map
    // println!("{}", field_name); <-- error
}

fn or_insert() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn update_value() {
    let text = "hello world wonderfull world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for pair in &scores {
        println!("{:?}", pair);
    }

    let vec = vec![1, 2];

    assert_eq!(vec[0], 1);

    hash_from_vectors();

    invalid_maps();

    or_insert();

    update_value();
}
