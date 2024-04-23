//Write a Rust function that returns a success value for positive numbers and an error value for negative numbers.


fn process_number(num: i32) -> Result<i32, &'static str> {
    if num >= 0 {
        Ok(num)
    } else {
        Err("Input number is negative")
    }
}

fn main() {
    let positive_num = 12;
    let negative_num = -10;

    match process_number(positive_num) {
        Ok(value) => println!("Success: {}", value),
        Err(err) => println!("Error: {}", err),
    }

    match process_number(negative_num) {
        Ok(value) => println!("Success: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}
