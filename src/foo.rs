pub fn functions() {
    // ? Functions in Rust

    another_function();
    value(5);
    print_labeled_measurement(5, 'h');
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
