fn main() {
    let n = 20;
    let mut fizzbuzz_count = 0;
    let mut fizz_count = 0;
    let mut buzz_count = 0;

    for i in 1..=n {
        if i > 15 {
            break;
        }

        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
            fizzbuzz_count += 1;
            continue;    
        } else if i % 3 == 0 {
            println!("Fizz");
            fizz_count += 1;
            continue;
        } else if i % 5 == 0 {
            println!("Buzz");
            buzz_count += 1;
            continue;
        }

        println!("{}", i);
    }

    println!("FizzBuzz count: {}", fizzbuzz_count);
    println!("Fizz count: {}", fizz_count);
    println!("Buzz count: {}", buzz_count);
}
