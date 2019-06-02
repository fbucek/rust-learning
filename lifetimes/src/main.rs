fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("Result is: {}", result);

    let novel = String::from("Novel text. Then next sentence");
    let first_sentence= novel.split(".")
        .next()
        .expect("Count not find a '.'");
    
    let i = ImportantExcerpt { part: first_sentence };

    println!("{:?}", i);
}
