// * importing a module in a file named foo.rs
// mod buffer;
mod foo;
mod loops;

fn main() {
    // foo::control();
    // foo::multi_conditions();
    // foo::iterator1();
    // let _ = buffer::main();
    // loops::break_with_value();
    loops::nested_loops();
}
