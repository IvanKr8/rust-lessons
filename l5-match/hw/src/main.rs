enum Message {
    Text(String),
    Number(i32),
    Coordinates { x: Option<i32>, y: Option<i32> },
    Status(u8),
}

fn match_string(text: &str) {
    match text {
        t if t.len() > 10 => println!("Long text: {}", t),
        _ => println!("Short text: {}", text),
    }
}

fn match_number(num: &i32) {
    match num {
        n @ 1..=10 => println!("small number"),
        n @ 11..=100 => println!("medium number"),
        _ => println!("big number"),
    }
}

fn match_coordinates(x: &Option<i32>, y: &Option<i32>) {
    match (x, y) {
        (Some(x), Some(y)) => println!("Point ({}, {})", x, y),
        (Some(x), None) => println!("X only: {}", x),
        (None, Some(y)) => println!("Y only: {}", y),
        (None, None) => println!("Empty point"),
    }
}

fn match_status(s: &u8) {
    match s {
        0 => println!("Offline"),
        1 => println!("Online"),
        2..=10 => println!("Busy"),
        _ => println!("Unknown status"),
    }
}

fn process(msg: &Message) {
    match msg {
        Message::Text(text) => match_string(text),
        Message::Number(num) => match_number(num),
        Message::Coordinates { x, y } => match_coordinates(x, y),
        Message::Status(s) => match_status(s),
    };
}

fn main() {
    // let message = Message::Text(String::from("Hello, world!"));
    // let message = Message::Number(191);
    // let message = Message::Coordinates { x: Some(5), y: Some(10) };
    let message = Message::Status(1);

    process(&message);
}