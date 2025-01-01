// ? while is a construct that behaves exactly like loop except that
// ? it automatically calls break when the condition ceases to be true

pub fn _condition() {
    let a = ["Elixir", "JavaScript", "Rust"];
    let mut index = 3;

    while index != 0 {
        index -= 1;
        println!("I will continue with learning: {}", a[index]);
    }
}

// * a for loop is actually less error prone than a while loop
// ? when looping through a collection
// * as a change in the data collection
// * will amount to changing the condition on the while loop as well

pub fn forcond() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// Prefer a for loop instead of a while loop

pub fn forrange(number: i32) {
    // the rightmost value in a range is not included as part of the range i.e to cover the value five
    // use 1..6
    for number in (1..number).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
