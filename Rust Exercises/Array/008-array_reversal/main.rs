

//Write a Rust program to create an array of characters with size 7 and initialize it with random characters. Reverse the elements of the array and print the reversed array.

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {

    //define arr with size of 7
    let mut arr: [char; 7] = ['a'; 7];
    
    //initilize randome char generator
    let mut rng = thread_rng();

    for i in 0..7 {
        // Generate a random alphanumeric character and convert it to char
        arr[i] = rng.sample(Alphanumeric) as char;
    }
    
    //reverse array
    arr.reverse();

    //print array
    println!("Reversed Array: {:?}", arr);
}