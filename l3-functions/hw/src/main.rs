fn parse_to_u32(s: &str) -> u32 {
    s.parse().unwrap()
}

fn get_status(age: u32) -> &'static str {
    if age >= 18 {
        "Adult"
    } else {
        "Minor"
    }
}

fn calculate_score(age: u32, height: u32) -> i32 {
    let mut score = 0;

    if age >= 18 {
        score += 10;
    }

    if height > 170 {
        score += 20;
    }

    score
}

fn main() {
    let age = "20";
    let height = "180";
    let name = "Ivan";

    let age = parse_to_u32(age);
    let height = parse_to_u32(height);

    let status = get_status(age);
    let score = calculate_score(age, height);

    println!("Name: {}, Age: {}, Height: {}, Status: {}, Score: {}", name, age, height, status, score);
}