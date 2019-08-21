mod sound;

mod plant {
    #[derive(Debug)]
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    #[derive(PartialEq)] // necessary to _order1 == menu::Appetizer::Salad {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use sound::instrument::clarinet;
use std::collections::HashMap;

fn main() {
    let mut v = plant::Vegetable::new("carrot");
    v.name = String::from("mrkev");
    println!("{} is delicious", v.name);
    println!("{:?}", v);

    let _order1 = menu::Appetizer::Salad;
    let order2 = menu::Appetizer::Soup;

    match order2 {
        menu::Appetizer::Salad => println!("Order 1 is: salad"),
        _ => println!("Order 1 is Soup"),
    }

    if _order1 == menu::Appetizer::Salad {
        println!("Apettizer is salad");
    }

    println!("Hello, Cargo!");
    // sound::instruments::clarinet();
    clarinet();

    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("{:?}", map);
}
