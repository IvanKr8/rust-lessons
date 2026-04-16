fn main() {
    let mut s = String::from("hello");

    // Slices is zero-cost (Borrowing) and safe (lifetime)
    let he = &s[..2];
    let llo = &s[2..];
    let hello = &s[..];

    println!("he: {}, llo: {}", he, llo);
    println!("hello: {}", hello);

    s.push_str(" world");

    println!("first word: {}, full: {}", first_word(&s), &s);

    let arr = [1, 2, 3, 4, 5];

    let arr_slice = &arr[1..3];

    println!("arr_slice: {:?}", arr_slice);
}

// Rust style to use slices instead of returning indices
fn first_word(s: &str) -> &str {
    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            return &s[..i];
        }
    }
    s
}
