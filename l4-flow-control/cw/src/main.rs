fn main() {
    let age = 20;

    // if-else 
    if age >= 18 {
        println!("You are an adult.");
    } else {
        println!("You are a minor.");
    }

    // if-else as expression
    let status = if age >= 18 {
        "Adult"
    } else {
        "Minor"
    };

    println!("Status: {}", status);

    let mut count = 0;

    // loop
    loop {
        count += 1;

        if count >= 5 {
            break;
        }
    }

    println!("Count: {}", count);

    // loop with return value
    let result = loop {
        break 10;
    };

    println!("Result: {}", result);

    // for 
    for i in 0..5 {
        println!("i: {}", i);
    }

    // for with inclusive range
    for i in 0..=5 {
        println!("i: {}", i);
    }

    // for with array
    let arr = [10, 20, 30, 40, 50];

    for x in arr {
        println!("x: {}", x);
    }

    // for with continue and break
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        if i > 7 {
            break;
        }
        println!("i: {}", i);
    }


}
