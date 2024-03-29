use great_calculator::*;


fn main() {
    hello();
    great_calculator::add(15f64,17f64);
}

///
/// ```
/// println!("this is test");
/// assert_eq(hello(), 15);
/// ```
fn hello() -> u32{
    println!("Hello, world!");
    15
}
