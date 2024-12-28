// * Rust has 3 kinds of loops :- loop, while and for

pub fn break_with_value() {
    let mut counter = 0;

    let result = loop {
        println!("The counter is {counter}");
        counter += 1;

        // * To return a value from a loop write the expression next to the break expressions 
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
