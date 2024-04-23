//Write a Rust program to create an array of integers with size 8 and initialize it with random values. Search for a specific value in the array and print whether it exists or not.

use rand::Rng;

fn main () {
    
    //define array
    let mut arr:[i32; 8] = [0; 8];
    
    //initialize randome number generator
    let mut rng = rand::thread_rng();
    
    //iterate over each index of an array
    for i in  0..8 {
        
        //generator random number
        arr[i] = rng.gen_range(1..101);
    }
    
    
    //define a search value 
    let search_value = 50;
    
    //print weather the value exists or not
    let exists = arr.iter().any(|&x| x == search_value);
    
    if exists {
        println!("the value {:?} exists in the arry", search_value);
    } else {
        println!("The vlaue {:?} does not exist in the array",search_value);
    }
 }