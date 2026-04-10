enum Direction {
    Up,
    Down,
}

// The 'match' has to be exhaustive, meaning all possible values must be covered by the patterns. If a value is not covered, the compiler will throw an error
fn match_number(num: i32) {
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        // The wildcard pattern `_` matches any value that hasn't been matched by previous patterns
        _ => println!("Other number"),
    }
}

fn main() {
    let num = 3;

    match_number(num);

    // Using match as an expression
    let result = match num {
        1 => "One",
        // Multiple patterns can be combined using the `|` operator
        2 | 3 => "Two or Three",
        // Ranges can be used to match a range of values
        4..=10 => "Between Four and Ten",
        _ => "Not One",
    };

    // The result of the match expression can be assigned to a variable
    let guard_result = match num {
        x if x % 2 == 0 => "Even number",
        x if x % 2 != 0 => "Odd number",
        _ => "Unknown",
    };

    // Matching on enums
    let dir = Direction::Up;

    match dir {
        Direction::Up => println!("Up"),
        Direction::Down => println!("Down"),
    }

    // Matching on Option
    let value = Some(10);

    match value {
        Some(x) => println!("Value: {}", x),
        None => println!("No value"),
    };

    enum User { 
        Admin(String),
        Guest,
    }

    let user = User::Admin(String::from("Ivan"));

    match user {
        User::Admin(name) => println!("Admin: {}", name),
        User::Guest => println!("Guest"),
    };

    struct Point {
        x: Option<i32>,
        y: Option<i32>,
    }

    let mut point = Point { x: None, y: None };

    match point {
        Point { x: Some(v), .. } => println!("Point with x: {}", v),
        Point { y: Some(v), .. } => println!("Point with y: {}", v),
        _ => {
            let point_x = 111;
            let point_y = 222;
            point = Point { x: Some(point_x), y: Some(point_y) };
            println!("Point initialized to ({}, {})", point_x, point_y);
        },
    };

    let text = Some(String::from("Hello"));

    match &text { // text is borrowed here, so we can still use it after the match {
        Some(v) => println!("Text: {}", v), // Some(v ref) would also work and would avoid the need for the & in the match statement
        None => println!("No text"),
    };

    // Ownership!
    println!("Text after match: {:?}", text);

    // Nested matches
    let nested_value = Some(Some(42));

    match nested_value {
        Some(Some(v)) => println!("Nested value: {}", v),
        _ => println!("No nested value"),
    };

    // Binding
    match num {
        x @ 1..=10 => println!("Number {} is between 1 and 10", x),
        _ => println!("Number is not between 1 and 10"),
    }

    if let Some(v) = text {
        println!("Text using if let: {}", v);
    }
}