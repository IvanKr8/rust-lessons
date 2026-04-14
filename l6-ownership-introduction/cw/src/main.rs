fn main() {
    // Move 
    // String is a heap-allocated type, so when we assign s1 to s2, s1 is moved to s2. Cannot be copied
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}", s1); // This will cause a compile-time error because s1 has been moved
    println!("{}", s2); // s2 is owner of the string now

    // Copy
    let a = 5;
    let b = a; // a is copied to b
    println!("a: {}, b: {}", a, b); // Both a and b can be used because i32 is lightweight type

    // Stack and Heap
    // Stack: (Lightweight types, can be copied) - i32, bool, char
    // Heap: (Complex types, cannot be copied) - String, Vec, HashMap

    // Borrowing
    // & - borrow
    let s3 = String::from("world");

    let len = calculate_length(&s3); // Borrow s3, not move

    println!("The length of '{}' is {}.", s3, len); // s3 can still be used because it was borrowed, not moved

    // Dereference
    let mut x = 10;
    let y = &mut x; // y is a reference to x

    println!("y: {}", *y); // Dereference y to get the value of x

    if *y > 5 {
        println!("y is greater than 5");
    }

    *y += 5;

    println!("y: {}", *y); // y is now 15
    println!("x: {}", x); // x is also 15 because y is a reference to x

    let mut str1 = String::from("Hello");
    let str2 = &str1; // read-only (immutable)
    let str3 = &mut str1; // writer (mutable)

    // This will cause a compile-time error because we cannot have both mutable and immutable references to the same data at the same time
    // println!("str2: {}, str3: {}", str2, str3); 

    // Ownership in functions
    let s4 = String::from("Hello, Rust!");
    take(s4); // s4 is moved to the function take

    // println!("{}", s4); // This will cause a compile-time error because s4 has been moved to the function take

    let num = 42;
    take_int(num); // num is copied to the function take_int

    println!("num: {}", num); // num can still be used because it was copied, not moved

    // Retunrning ownership from functions
    let s5 = String::from("Hello, ownership!");
    let s6 = take_and_return(s5);

    println!("s6: {}", s6); // s6 is now the owner of the string returned from the function

    // Cloning 
    let s7 = String::from("Hello, clone!");
    let s8 = s7.clone(); // s7 is cloned to s8
    println!("s7: {}, s8: {}", s7, s8); // Both s7 and s8 can be used, and they are independent heap allocations
} // <- Ownership goes out of scope here, and the memory is freed

fn calculate_length(s: &String) -> usize {
    s.len() // Can use s because its borrowed, not moved 
}

fn take(s: String) {
    println!("{}", s);
}

fn take_int(num: i32) {
    println!("{}", num);
}

fn take_and_return(s: String) -> String {
    println!("{}", s);
    s // Return s, transferring ownership back to the caller
}
