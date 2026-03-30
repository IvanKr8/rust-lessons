struct MyStruct {
    field1: i32,
    field2: f64,
    field3: char,
}

fn main() {
    //! int:
    // - i8, i16, i32, i64, i128
    let a_i8: i8 = 127;
    let a_i16: i16 = 32767;
    let a_i32: i32 = 2147483647;
    let a_i64: i64 = 9223372036854775807;
    let a_i128: i128 = 170141183460469231731687303715733975807;

    //! uint:
    // - u8, u16, u32, u64, u128
    let a_u8: u8 = 255;
    let a_u16: u16 = 65535;
    let a_u32: u32 = 4294967295;
    let a_u64: u64 = 18446744073709551615;
    let a_u128: u128 = 340282366920938463463374607431768211455;

    //! float:
    // - f32, f64
    let a_f32: f32 = 3.14;
    let a_f64: f64 = 3.141592653589793;

    //! char:
    let a_char: char = 'A';

    //! bool:
    let a_bool: bool = true;

    //! unit:
    let a_unit: () = ();

    //! array:
    let a_array: [i32; 5] = [1, 2, 3, 4, 5];

    //! slice:
    let a_slice: &[i32] = &[1, 2, 3, 4, 5];

    //! string:
    let a_string: String = String::from("Hello, Rust!");
    //! str:
    let a_str: &str = "Hello, Rust!";

    //! tuple:
    let a_tuple: (i32, f64, char) = (42, 3.14, 'A');

    //! Option:
    let a_option: Option<i32> = Some(42);
    let a_none: Option<i32> = None;

    // Option Example:
    fn example_option(value: Option<i32>) {
        match value {
            Some(v) => println!("Value is: {}", v),
            None => println!("Value is None"),
        }
    }

    //! Result:
    let a_result: Result<i32, String> = Ok(42);
    let a_error: Result<i32, String> = Err(String::from("An error occurred"));

    // Result Example:
    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }

    let result = divide(10, 0);

    match result {
        Ok(value) => println!("Result is: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    //! enum:
    enum MyEnum {
        Up,
        Down,
        Left,
        Right,
    }

    // Enum Example:
    let dir = MyEnum::Up;

    fn example_enum(direction: MyEnum) -> Result<(), String> {
        match direction {
            MyEnum::Up => println!("Direction is Up"),
            MyEnum::Down => println!("Direction is Down"),
            MyEnum::Left => println!("Direction is Left"),
            MyEnum::Right => println!("Direction is Right"),
        }
    }

    match example_enum(dir) {
        Ok(_) => println!("Enum example executed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
