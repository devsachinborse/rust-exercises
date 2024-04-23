
//Write a Rust program to create two arrays of integers, each with size 4, and initialize them with random values. Concatenate the two arrays and print the resulting array.

use rand::Rng;

fn main() {
    
    let mut arr1: [i32; 4] = [0;4];
    let mut arr2: [i32; 4] = [0;4];
    
    let mut rng = rand::thread_rng();
    
    for i in 0..4 {
        arr1[i] = rng.gen_range(0..101);
    }
    
     for i in 0..4 {
        arr2[i] = rng.gen_range(0..101);
    }
    
    //concatenate the arrays
    let mut con_array : [i32; 8] = [0;8];
    
    
    for i in 0..4 {
        con_array[i] = arr1[i];
        con_array[i + 4] = arr2[i] ;
    }
    
    
    //print con array
    println!("concatenate array : {:?}",con_array );
    
    
}
