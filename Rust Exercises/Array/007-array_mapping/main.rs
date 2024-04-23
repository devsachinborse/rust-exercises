
//Write a Rust program to create an array of integers with size 5 and initialize it with random values. Map each element of the array to its square and print the resulting array

use rand::Rng;

fn main() {
    
    //define arr with size of 5
    let mut arr :[i32; 5] = [0;5];
    
    //initialize the randome number generator
    let mut rng = rand::thread_rng();
    
    //iterate ove each index
    for i in 0..5{
        arr[i] = rng.gen_range(0..101);
        
    }
    
    //map each element of the array to its square
    let squared_arr : Vec<i32> = arr.iter()// conv array into an iterator
    .map(|&x| x * x) //map method to square each elemt of the iterator
    .collect(); //collect the squared ele into a new vector
    
    
    //print new array with squared ele
    println!("squared array: {:?} ", squared_arr);
}