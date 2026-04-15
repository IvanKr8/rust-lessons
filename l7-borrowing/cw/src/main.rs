

fn main() {
    // Immutable references (read-only)

    let name = String::from("Ivan");
    // Multiple immutable references are allowed
    let name_ref = &name; // borrow, not move
    let name_ref2 = &name;

    println!("Name: {}, Name Ref: {}, Name Ref2: {}", name, name_ref, name_ref2);

    // Mutable references (write access)
    let mut s = String::from("Hello");
    // Multiple mutable references are NOT allowed
    let r1 = &mut s; // mutable borrow
    // immutable borrow - this will cause a compile-time error:
    // let r2 = &s; 

    r1.push_str(" World!");

    println!("Mutable Reference: {}", r1);

    // Deference
    let mut x = 5;
    let y = &mut x;

    // Rust automatically dereferences
    *y += 1;

    // This will cause a compile-time error because y is a mutable reference and cannot be used after being dereferenced:
    // println!("Value of x: {}, Value of y: {}", x, y);

    println!("Value of y: {}", y);
    println!("Value of x: {}", x);

}

fn print_length(s: &String) {
    println!("The string is: {} length: {}", s, s.len());
}

fn change(s: &mut String) {
    s.push_str("!!");
}
