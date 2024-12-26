// use rand::{thread_rng, Rng};


pub fn variables() {

        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
        // let secret_number = rand::thread_rng().gen_range(1..=100);
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;

        // * a Constant must always be assigned to a constant expression 
        // * this will fail 
        // * constants are always immutable
        // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * secret_number;
        
        println!("Four hours in seconds is: {THREE_HOURS_IN_SECONDS}");

}
