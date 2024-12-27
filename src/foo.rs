// use rand::{thread_rng, Rng};
// * Using constants in Rust is indeed helpful as
// * it helps Us adhere to the DRY principle
// * and in case something changes we only modify it at that particular place
// * module attributes in Elixir play the same role except they are discarded after compilation

pub fn variables() {
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");
    // let secret_number = rand::thread_rng().gen_range(1..=100);
    const FOUR_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;
    // * a Constant must always be assigned to a constant expression
    // * this will fail because
    // * constants are always immutable
    // const FOUR_HOURS_IN_SECONDS: u32 = 60 * 60 * secret_number;
    println!("Four hours in seconds is: {FOUR_HOURS_IN_SECONDS}");
    let x = 5;
    println!("The value of x here is: {x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let _spaces = spaces.len();
    println!(
        "Shadowing is effectively creating a new 
        variable with the same name as the 
        previous one meaning the previous value will be discarded
        Using shadowing also means that the variable will still remain immutable 
        after the set of operations has been completed"
    )
}
