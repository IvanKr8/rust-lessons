fn parse(input: &str) -> Vec<(&str, &str)> {
    let mut result = Vec::new();

    let pairs = input.split(' ');
    for pair in pairs {
        let mut iter = pair.split('=');

        if let (Some(key), Some(value)) = (iter.next(), iter.next()) {
            if !key.is_empty() {
                result.push((key, value));
            }
            
        }
    }
    result
}

fn get_value<'a>(input: &'a str, key: &str) -> Option<&'a str> {
    let pairs = input.split(' ');
    for pair in pairs {
        let mut iter = pair.split('=');

        if let (Some(k), Some(v)) = (iter.next(), iter.next()) {
            if k == key {
                return Some(v);
            }
        }
    }
    None
}

fn first_value(input: &str) -> Option<&str> {
    let pairs = input.split(' ');
    for pair in pairs {
        let mut iter = pair.split('=');

        if let (Some(k), Some(v)) = (iter.next(), iter.next()) {
            if !k.is_empty() {
                return Some(v);
            }
        }
    }
    None
}

fn print_pairs(pairs: Vec<(&str, &str)>) {
    for (key, value) in pairs {
        println!("{}: {}", key, value);
    }
}

fn main() {
    let s = "name=Ivan age=17 invalid test_data! = city=Frankfurt";

    let pairs = parse(s);

    match pairs.len() {
        0 => println!("No pairs found"),
        _ => print_pairs(pairs),
    }

    if let Some(name) = get_value(s, "name") {
        println!("Name: {}", name);
    } else {
        println!("Name not found");
    }

    if let Some(first) = first_value(s) {
        println!("First value: {}", first);
    } else {
        println!("No values found");
    }
}
