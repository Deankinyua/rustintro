// * The presence of the ! indicates a you are calling a macro

// * Most lines of Rust end in a semicolon
// * cargo check, cargo run and cargo build

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
