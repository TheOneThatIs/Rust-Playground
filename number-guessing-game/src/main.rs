#![allow(unused)]

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("*****GUESSING GAME*****");

    let answer = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Input guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number!");
                continue;
            },
        };

        println!("Your guess is: {guess}");

        match guess.cmp(&answer) {
            Ordering::Less => println!("You guessed too low...\n"),
            Ordering::Greater => println!("You guessed too high...\n"),
            Ordering::Equal => {
                println!("You guessed correctly!\n");
                break;
            }
        }
        

    }
}
