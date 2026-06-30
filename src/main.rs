//! Starter REPL

use std::io;

/// Simple echos only
fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                break
            }
            Ok(_) => {
                match input.trim() {
                    "exit" => break,
                    _ => println!("{input}"),
                }
            }
            Err(error) => {
                println!("Um {error}, maybe try again");
                continue
            }
        }
    }
}
