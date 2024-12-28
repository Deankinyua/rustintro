pub fn functions() {
    // ? Functions in Rust

    another_function();
    value(5);
    print_labeled_measurement(5, 'h');
    block_expression();

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

// You must include the type of the parameter in the function definition
fn value(x: i32) {
    println!("The value of x is: {x}");
}

// With multiple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Understanding statements and expressions in Rust

// ? Statements are instructions that perform some action and do not return a value.
// ? Expressions evaluate to a resultant value.

// expressions return values e.g x + 2
// statements do not !! e.g declaring a variable and assigning it to a value is a statement
// fn statement_example() {
// let x = (let y = 6);
// }

// * Just like in Elixir, the last expression is the return value of the function
fn block_expression() {
    let y = {
        let x = 3;
        // Expressions do not end in semicolons
        x + 1
    };

    println!("The value of y is: {y}");
}

// Declare a function's return type using the -> after parenthesis
// When we declare a return type, we are able to know if something goes wrong during execution

fn plus_one(x: i32) -> i32 {
    x + 1
}
