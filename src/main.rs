use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let answer = rand::thread_rng().gen_range(0..=100);
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
         io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Not a number!");
        println!("You guessed: {guess}");

        match guess.cmp(&answer) {
            Ordering::Equal => {println!("You win!"); break; },
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
        }
    }
}