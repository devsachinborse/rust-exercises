//Write a Rust program that declares a variable birth_year and calculates the age based on the current year and the user's birth year.

use std::io;

fn main () {
    println!("Enter your birth year: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Faild to read line");

    let birth_year : u32 = input.trim().parse().expect("Invalid input");

    let current_year = 2024 ;//define current year;

    let age = current_year - birth_year;
    println!("Your age is {} years old.", age);
}