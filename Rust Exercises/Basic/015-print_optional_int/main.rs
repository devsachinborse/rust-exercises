//Write a Rust function that takes an optional integer and prints its value if it exists

fn print_optional_int(opt_int: Option<i32>) {
    match opt_int {
        Some(value) => println!("The value is: {}", value),
        None => println!("The value is None."),
    }
}

fn main() {
    let some_value = Some(42);
    let none_value: Option<i32> = None;

    print_optional_int(some_value);
    print_optional_int(none_value);
}
