// ? The formula is (F - 32) * 5 / 9
// -459F is the lowest possible Farenheit value

pub fn convert(temperature: f64) {
    if temperature < -459.0 {
        println!("Temperature is too low to be computed");
    } else {
        let mut degrees = temperature - 32.00;

        degrees = degrees * (5.00 / 9.00);

        println!("The temperature in degress celcius is : {degrees}");
    }
}

pub fn fibonacci(n: u32) {
    if n == 0 {
        return ();
    }
    let mut a = 0;
    let mut b = 1;

    for _ in 1..n {
        // the fibonacci is the sum of a and b
        let big_num = a + b;
        // make a the number before the fibonacci
        a = b;
        // make b to be the current fibonacci
        b = big_num;
    }

    println!("The fibonacci is : {b}");
}
