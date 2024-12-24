// * a crate is a single compilation unit in Rust
// * a crate can be compiled into a library or a binary
// * The presence of the ! indicates that you are calling a macro

// * Most lines of Rust end in a semicolon
// * cargo check, cargo run and cargo build

// If a type you want to use isn’t in the prelude,
// you have to bring that type into scope explicitly with a use statement.

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    // * by default values in Rust are immutable but
    // * we can use the mut keyword to override this behaviour
    // * As you can see Rust is statically and Strongly typed

    let mut guess = String::new();

    // the Rust analyzer adds the type declaration on the left side

    // it seems that instead of using the dot syntax to call functions rust relies on the :: syntax

    io::stdin()
        // &mut guess is a reference
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

// importing a module in a file named foo.rs
// mod foo;

// fn main() {
//     foo::hello();
// }
