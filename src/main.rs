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
    airthemtical_operator();

    // assignment operator
    println!("===============assignment operator================");
    assignment_operator();

    println!("=============comparison operator=================");
    comparison_operator();

    println!("=============logical operator==============");
    logical_operator();

    println!("==============Conditional if ================");
    conditional_if();

    println!("==============Conditional switch ================");
    conditional_match();

    println!("=================Looping==================");
    forever_loop();
    while_loop();
    for_loop(6);

    println!("========Different ways to store String===========");
    changeable_string();

    println!("=======Onwership and borrowing=======");
    ownership_borrowing();
}

fn airthemtical_operator() {
    let num1 = 10;
    let num2 = 20;

    println!("Addition {}", num2 + num1);
    println!("Subtract {}", num2 - num1);
    println!("Multiply {}", num2 * num1);
    println!("Divide {}", num2 / num1);
    println!("Modulo {}", num2 % num1);
}

fn assignment_operator() {
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
}

fn comparison_operator() {
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);
}

fn logical_operator() {
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);
}

fn conditional_if() {
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
}
fn conditional_match() {
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
}

fn forever_loop() {
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
}

fn while_loop() {
    let mut count = 0;
    // while loop till the condition is true
    while count <= 10 {
        println!("while loop count {}", count);
        count += 1;
    }
}

fn for_loop(till: i32) {
    // for loop when you know exact value of iteration
    for i in 1..till {
        println!("for loop {}", i)
    }

    for i in 1..=till {
        println!("for loop inclusive {}", i)
    }
}

fn changeable_string() {
    let s1: &str = "Hello"; // this is string slice
    let mut s2 = String::from("World"); // this is changeable string
    let s3 = format!("{} {}", s1, s2); // interpolate string, can also use + operator for same
    println!("String Formatting {} length of string {}", s3, s3.len());

    s2.push_str("people");
    println!("{}", s2);

    s2.push('A'); // single character
    println!("{}", s2);
}

fn ownership_borrowing() {
    let a = 10; //ownership
    let b = a; // b become the owner of 10 and a still remain
    println!("b value {}", b);

    let s1 = String::from("Hello");
    let s2 = s1; // s2 become the owner and s1 no longer exist
    // println!("{}", s1); // error s1 moved
    let s3 = s2.clone(); // s3 is the owner of "Hello", and s2 also exist
    println!("{} {}", s2, s3);

    // borrowing
    let mut s4 = String::from("Hello");
    let s5 = &s4; // this is borrowing

    println!("borrowing {}", s5);
    let s6 = &mut s4;
    s6.push_str(" updating world");
    println!("{}", s6);
}
