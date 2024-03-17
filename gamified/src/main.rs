use std::io;
use rand::Rng;

fn main() {
    // Generate a random secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to the Number Guessing Game!");
    println!("I've picked a secret number between 1 and 100.");
    println!("Can you guess it?");

    loop {
        // Read the player's guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! You win!");
                break;
            }
        }
    }
}
