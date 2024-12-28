// * Control Flow deals with if expressions, loops etc
use std::env;

pub fn _control() {
    // Blocks of code associated with conditions are sometimes called arms
    //  just like arms in match expressions
    // It is important to note that the value in if expressions MUST always be a Boolean
    let number = 3;

    // * Also you cannot compare values of different types

    // e.g number != false will fail

    if number != 0 {
        println!("number was something other than zero");
    }
}

pub fn _multi_conditions() {
    let number = 13;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

pub fn iterator1() {
    // Prints each argument on a separate line
    for argument in env::args() {
        println!("{}", argument);
    }
}
