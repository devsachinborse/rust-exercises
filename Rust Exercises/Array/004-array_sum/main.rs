//Write a Rust program to create an array of integers with size 7 and initialize it with random values. Calculate and print the sum of all the elements in the array.


use rand::Rng; //import rand to create random numbers

fn main () {
    
    //define an aray and initalize with random values
    let mut arr : [i32; 7] = [0; 7];
    
    //initialize the random number generator
    let mut rng = rand::thread_rng();
    
    //iterate ove each index of the arry
    for i in 0..7 {
        arr[i] = rng.gen_range(1..101);
    }
    
    //cal the sum of all elements
    let sum: i32 = arr.iter().sum();
    
    //print sum of all random elemets in the array
    println!("sum of all elemets of the random numbers: {:?}", sum);
}
