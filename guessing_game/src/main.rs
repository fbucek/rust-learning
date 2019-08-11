use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn test() -> i32 {
    5
}

#[derive(Debug)]
struct Soap {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = Soap {
        email: String::from("user@email.com"),
        username: String::from("user"),
        active: true,
        sign_in_count: 1,
    };

    println!("Soup is: {:?}", user1);
    println!("test return: {}", test());

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read file");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            //.expect("Please type a number!");
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
