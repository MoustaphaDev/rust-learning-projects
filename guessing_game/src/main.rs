use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse input. Make sure to type in a number!");
                continue;
            }
        };

        print!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!(", it's too small!"),
            Ordering::Greater => println!(", it's too big!"),
            Ordering::Equal => {
                println!("\nYou win!");
                break;
            }
        }
    }
}
