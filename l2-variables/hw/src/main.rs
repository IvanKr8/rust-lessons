const MAX_SCORE: i32 = 100;

fn get_status(age: u32) -> Result<&'static str, &'static str> {
    if age >= 18 {
        Ok("Adult")
    } else {
        Ok("Minor")
    }
}

fn main() {
    let name: &str = "Ivan";
    let age: &str = "18";
    let height: &str = "180";

    let age: u32 = age.parse().unwrap();
    let height: u32 = height.parse().unwrap();

    let mut score = 0;
    score += 10;
    score += 20;

    {
        let age = age + 1;
        println!("age in scope is: {}", age);
    }

    println!("current age is: {}", age);

    let status = match get_status(age) {
        Ok(s) => s,
        Err(_) => "Unknown",
    };
    
    println!("name: {}, age: {}, height: {}, score: {}, status: {}, MAX_SCORE: {}", name, age, height, score, status, MAX_SCORE);
}
