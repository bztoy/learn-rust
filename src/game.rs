use rand::Rng;
use std::cmp::Ordering;
use std::io;

use crate::utils::print_close_paragraph;
use crate::utils::print_topic_title;

pub fn guessing_game() {
    print_topic_title("Guess the number!");
    //println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Input your guess:");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");
        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!!"),
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
        }
    }
    print_close_paragraph();
}
