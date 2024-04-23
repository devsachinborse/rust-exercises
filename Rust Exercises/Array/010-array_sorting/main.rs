//Write a Rust program to create an array of integers with size 10 and initialize it with random values. Sort the array in ascending order and print the sorted array.

use rand::Rng;

fn main() {
    
    let mut arr: [i32; 10] = [0;10];

    let mut rng = rand::thread_rng();
    
    for i in 0..10 {
        arr[i] = rng.gen_range(0..101);
    }
    
    //sort the array in ascending order
    arr.sort() ;

    
    //print con array
    println!("concatenate array : {:?}",arr );
    
}