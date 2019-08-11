#[derive(Debug)]
pub enum UsState {
    Alabama,
    Alaska,
    Hawaii,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum() {
        //let coint = Coin::Quarter(UsState::Alabama));
        assert_eq!(value_in_cents(Coin::Quarter(UsState::Alabama)), 25);
    }
}