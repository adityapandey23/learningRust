use std::{cmp::Ordering, io};
use rand::{rng, Rng};

fn main() {
    println!("Guess the number!");

    let secret_number = rng().random_range(1..=100);
    
    loop {
        println!("Please input your guess");
        let mut guess: String = String::new(); // Concatenate it, doesn't overwrite it
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        // let guess: u32 = guess.trim().parse().expect("Please type a number"); // 45/n -> that's why trim is needed

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                println!("Please enter a valid input");
                continue;
            },
        };

        println!("Your guess is {guess}");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
