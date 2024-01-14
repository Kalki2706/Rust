use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=20); // generating random number

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // storing user input in String type

        // taking user input
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // crash program when error occurs

        // converting user input from String to number
        let guess: u32 = match guess.trim().parse() { // changed expect to match, for handling the error instead of crashing.
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        // comparing user input with random number
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
