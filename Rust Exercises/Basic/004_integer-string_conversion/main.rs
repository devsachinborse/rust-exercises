fn main() {
    // Convert integer to string
    let integer_value = 42;
    let string_value = integer_value.to_string();
    println!("Integer to string: {}", string_value);

    // Convert string to integer
    let string_value = "123";
    let parsed_integer: Result<i32, _> = string_value.parse();
    match parsed_integer {
        Ok(int_value) => println!("String to integer: {}", int_value),
        Err(_) => println!("Failed to parse string to integer"),
    }
}
