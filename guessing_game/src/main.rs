use rand::Rng;
use std::{cmp::Ordering, io};

mod guards {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Self {
            if !(1..=100).contains(&value) {
                panic!("The number should be between 1 and 100");
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

use crate::guards::Guess;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse input. Make sure to type in a number!");
                continue;
            }
        };

        let guess = Guess::new(guess);

        print!("You guessed {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!(", it's too small!"),
            Ordering::Greater => println!(", it's too big!"),
            Ordering::Equal => {
                println!("\nYou win!");
                break;
            }
        }
    }
}
