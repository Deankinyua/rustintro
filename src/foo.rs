pub fn variables() {
    let x = 2.5555555669484949; // f64

    let y: f64 = 3.000344; // f64
    let _quotient = 56.7 / 32.2;

    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    let _t = true;
    println!("The multiplication of {x} and {y} is {}", x * y);

    // This is a tuple in Rust ....more like an array in JavaScript
    // ? Tuples cannot grow or shrink in size and have a fixed length

    let tup = (500, 6.4, 1);

    // * Pattern Matching used for destructuring just like in Elixir
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let tuple2 = ("Kerware", 94, "Makueni Boys");

    // Tuples can also be accessed by their index as seen below

    println!("The value of the first element in Tuple 2 is {}", tuple2.0);
    println!("The value of the second element in Tuple 2 is {}", tuple2.1);

    // * Arrays in Rust must have a fixed length and the elements
    // * inside an Array must be of the same type
    let _a = [1, 2, 3, 4, 5];
    // * Store data that is of a predefined length
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
