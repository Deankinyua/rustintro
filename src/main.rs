// * a crate is a single compilation unit in Rust
// * a crate can be compiled into either a library or a binary
// * The presence of the ! indicates that you are calling a macro

// * Most lines of Rust end in a semicolon
// * cargo check, cargo run, cargo doc --open and cargo build

// If a type you want to use isn’t in the prelude,
// you have to bring that type into scope explicitly with a use statement.

// use rand::Rng;
use std::cmp::Ordering;
use std::io;

use rand::{thread_rng, Rng};

fn main() {
    // thread_rng returns the specific number generator
    let mut rng = thread_rng();

    // Exclusive range
    let n: u32 = rng.gen_range(2..4);
    println!("{}", n);
    let m: f64 = rng.gen_range(-40.0..1.3e5);
    println!("{}", m);

    // Inclusive range
    let n: u32 = rng.gen_range(2..=4);
    println!("{}", n);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        // * by default values in Rust are immutable but
        // * we can use the mut keyword to override this default behaviour
        // * As you can see Rust is statically and Strongly typed

        //* Determining the type by inference
        let mut guess = String::new();

        // the Rust analyzer adds the type declaration on the left side

        // it seems that instead of using the dot syntax to call functions rust relies on the :: syntax

        io::stdin()
            // The & indicates that this argument is a reference
            .read_line(&mut guess)
            .expect("Failed to read line");

        // * Enforcing the type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // the match expression in Rust is almost the same as the case declaration in Elixir

        match guess.cmp(&secret_number) {
            // is the guess less than the secret number ?
            Ordering::Less => println!("Too small!"),
            // is the guess greater than the secret number ?
            Ordering::Greater => println!("Too big!"),
            // is the guess equal to the secret number ?
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // let x = 5;
    // let y = 10;

    // println!("x = {x} and y + 2 = {}", y + 2);
}

// importing a module in a file named foo.rs
// mod foo;

// fn main() {
//     foo::hello();
// }
