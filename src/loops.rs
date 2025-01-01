// ? Rust has 3 kinds of loops :- loop, while and for

// ? understand that with loop code is executed indefinitely until you explicitly tell it to stop
// ? with the Break expression

// lets think of when you probably are waiting for a result from another process
// You will keep checking (loop) until you get what you're looking for then break
pub fn _break_with_value() {
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

// * It is a good idea to name your loops if you
// * are nesting them to allow for selective break and continue
pub fn _nested_loops() {
    let mut count = 0;
    // count is incrementing
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        // remaining is decrementing

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // * By default when you have nested loops
                // * break and continue apply to the innermost loop at that point.
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
