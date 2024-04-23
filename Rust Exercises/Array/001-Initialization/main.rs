//Write a Rust program to create an array of integers with size 7 and initialize it with values 1, 2, 3, 4, 5, 6 and 7. Print the array.



fn main () {
    
    //define arra with size of 7 
    let arr : [i32; 7] = [1,2,3,4,5,6,7];
    
    //print array
    println!("Array: {:?},", arr);
}