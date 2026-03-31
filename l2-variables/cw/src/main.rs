// ! CONSTANT
// Naming: UPPER_SNAKE_CASE
// Type is required
// Immutable
const MAX_POINTS: u32 = 100_000;

fn main() {
    // ! MUTABILITY
    // By default variables are immutable
    let x = 10;
    println!("value x: {}", x);

    // ! SHADOWING
    // Reusing the same variable name with a new value/type
    let im = "32";
    let im: i32 = im.parse().unwrap();
    println!("value im: {}", im);

    // mut allows changing the same variable
    // but here we still use shadowing (new variable)
    let mut m = "32";
    let m: i32 = m.parse().unwrap();
    println!("value m: {}", m);

    let mut y = 10;
    y = 20;
    println!("value y: {}", y);

    // ! SHADOWING (example)
    let z = 10;
    let z = z + 5;
    println!("value z: {}", z);

    // ! SCOPE
    // Variables exist only inside their block
    let a = 10;

    {
        let a = 20;
        println!("inner a: {}", a);
    }

    println!("outer a: {}", a);
}