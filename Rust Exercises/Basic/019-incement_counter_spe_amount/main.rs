
//Write a Rust function that accepts a mutable reference to a counter variable and increments it by a specified amount.
//Increment counter by specified amount

//define a function
fn increment_counter(counter: &mut i32, amount: i32) {
    *counter += amount; //increment the counter
}

fn main () {

    let mut counter = 2;
    let amount = 7;

    increment_counter(&mut counter, amount);

    println!("Counter after increment:  {}", counter);
}