mod coin;

use coin::{
    Coin,
};


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    let some_u8_value: u8 = 3;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }

    let some_value = Some(3u8);
    match some_value {
        Some(3) => println!("three second"),
        _ => (),
    }

    let mut count = 0;
    let coin_coin = coin::Coin::Quarter(coin::UsState::Hawaii);
    match coin_coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }


    {
        let coin_coin = coin::Coin::Quarter(coin::UsState::Hawaii);
        let mut count = 0;
        if let Coin::Quarter(state) = coin_coin {
            println!("State quarter from {:?}", state);
        } else {
            count += 1;
        }
    }


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five {}, six {}", five.unwrap(), six.unwrap());


    let penny = Coin::Penny;
    let quarter = Coin::Quarter(coin::UsState::Hawaii);

    println!("{}", coin::value_in_cents(penny));
    println!("{}", coin::value_in_cents(quarter));

    println!("Hello, world!");
}

