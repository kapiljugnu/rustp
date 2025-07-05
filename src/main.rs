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

    println!("==============Conditional if ================");
    if true {
        println!("Will always execute");
    }

    if 6 == 7 {
        println!("never execute");
    } else {
        println!("else execute")
    }

    // unary expression in other language
    let message = if true {
        "if as expression "
    } else {
        "always require else"
    };
    println!("{}", message);

    if 6 == 7 {
        println!("never execute");
    } else if 6 == 6 {
        println!("will execute");
    } else {
        println!("never execute");
    }

    println!("==============Conditional switch ================");
    // switch case
    let num = 3;
    match num {
        1 => println!("Number 1"),
        2 => println!("Number 2"),
        3 => println!("Number 3"),
        _ => println!("default case"),
    };

    // switch case expression
    let switch_message = match num {
        1 => "Number 1",
        2 => "Number 2",
        3 => "Number 3",
        _ => "Default case",
    };

    println!("switch case expression :: {}", switch_message);

    println!("Multiple matches");
    //multiple case
    match num {
        1 | 2 => println!("Number 1 | 2"),
        3 | 4 => println!("Number 3 | 4"),
        _ => println!("default case"),
    };

    println!("=================Looping==================");
    let mut count = 0;

    // will run forever if not used break
    loop {
        println!("loop number {}", count);

        if count == 3 {
            break;
        }

        count += 1;
    }

    // loop as expression
    let _result = loop {
        println!("loop number {}", count);

        if count == 6 {
            break;
        }

        count += 1;
    }; // semicolon is necessary for expression

    // println!("final result {}", result) // not sure it is not working

    // while loop till the condition is true
    while count <= 10 {
        println!("while loop count {}", count);
        count += 1;
    }

    // for loop when you know exact value of iteration
    for i in 1..6 {
        println!("for loop {}", i)
    }

    for i in 1..=6 {
        println!("for loop inclusive {}", i)
    }
}
