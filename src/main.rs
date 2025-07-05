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

    // arithmetic operators
    println!("==============airthmetic operator==================");
    let num1 = 10;
    let num2 = 20;

    println!("Addition {}", num2 + num1);
    println!("Subtract {}", num2 - num1);
    println!("Multiply {}", num2 * num1);
    println!("Divide {}", num2 / num1);
    println!("Modulo {}", num2 % num1);

    // assignment operator
    println!("===============assignment operator================");
    let mut ass_num = 1;
    ass_num += 5;
    println!("Addition {}", ass_num);

    ass_num -= 1;
    println!("Subtraction {}", ass_num);

    ass_num /= 2;
    println!("Divide {}", ass_num);

    ass_num *= 4;
    println!("Multiply {}", ass_num);

    ass_num %= 3;
    println!("Modulo {}", ass_num);

    println!("=============comparison operator=================");
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);

    println!("=============logical operator==============");
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);
}
