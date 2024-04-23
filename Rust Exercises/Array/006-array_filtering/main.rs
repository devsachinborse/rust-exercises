//Write a Rust program to create an array of integers with size 9 and initialize it with random values. Filter out even numbers from the array and print the resulting array

use rand::Rng;

fn main () {
    
    //define array with the size of 9
    let mut arr : [i32; 9] = [0; 9];
    
    //initialize the random number generator
    let mut rng = rand::thread_rng();
    
    //iterate over each index
    for i in 0..9 {
        arr[i] = rng.gen_range(1..101);
    }
    
    //filter out even and odd numbers from the array
    let filtered_array : Vec<i32> = arr.iter()//convert array into the iterator
    .copied()//make a copy of each elemet of the iterator
    .filter(|&x| x%2 != 0) // Use the filter method to keep only elements that are not even 
    .collect();  // Collect the filtered elements into a new vector
    
    //print filtered array
    println!("filtered Array: {:?}", filtered_array);
}