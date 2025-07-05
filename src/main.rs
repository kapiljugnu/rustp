fn main() {
    println!("Hello, world!");
    print!("Hello, world!\n"); //alternative to println! macro, have to explicitly add new line character \n
    let x = 10; // immutable/non changeable variable
    let mut y = 10; // mutable/changeable variable
    println!("variables values x={} y={}", x, y); // string with placeholder, use double quotes
    y = 20;
    println!("y value {}", y);
    const CONST_REQUIRE_TYPE: &str = "CONST REQUIRE TYPE"; // variable can be assign type but not mandatory
    println!("{}", CONST_REQUIRE_TYPE);
}
