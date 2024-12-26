pub fn hello() {
    let number1: i64 = -445;

    let y1: f64 = 3.09979797577483393930006099;
    let y2: f32 = 3.09979797577483393930006099;
    let truncated  = -5 / 3;

    println!("the number is a signed integer of size 32 - {number1}");
    println!("32 bit float has less precision - {y2}");
    println!("than 64 bit float {y1}");
    println!("Quotient is : {truncated}");
}
