pub fn functions() {
    // ? Functions in Rust

    another_function();
    value(5);
    print_labeled_measurement(5, 'h');
    block_expression();
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

fn block_expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
