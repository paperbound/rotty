//! Super special program

use std::cmp::Ordering;
use std::io;

/// Simple program to guess a number between 0 and a 100
fn main() {
    let pick: u8 = rand::random_range(0..=100);
    println!("Sh!!! Picked {pick}");
    println!("Please take make a guess");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read a number");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Um, maybe guess again");
                continue
            }
        };

        match guess.cmp(& pick) {
            Ordering::Less => println!("Your guess is too low"),
            Ordering::Greater => println!("Your guess is too high"),
            Ordering::Equal => {println!("Exactomondo!"); break;}
        }
    }
}
