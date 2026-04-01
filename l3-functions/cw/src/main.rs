
//! main Function - programm entry
fn main() {
    println!("Hello, world!");
    greet("Ivan");

    let c: i32 = add(2, 3);
    println!("2 + 3 = {}", c);

    let d: i32 = add_with_return(5, 7);
    println!("5 + 7 = {}", d);

    let x = {
        let a = 10;
        let b = 20;
        a + b
    };

    println!("x = {}", x);

    println!("Is 20 years old an adult? {}", is_adult(20));

    println!("Status of 17 years old: {}", get_status(17));

    let name: &str = "Ivan";
    println!("Before print_name: {}", name);
    print_name(&name);
    println!("After print_name: {}", name);
}

fn greet(name: &str) {
    println!("hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn add_with_return(a: i32, b: i32) -> i32 {
    return a + b;
}

fn is_adult(age: u8) -> bool {
    age >= 18
}

fn get_status(age: u8) -> &'static str {
    if age >= 18 {
        "Adult"
    } else {
        "Minor"
    }
}

fn print_name(name: &str) {
    println!("Name: {}", name);
}