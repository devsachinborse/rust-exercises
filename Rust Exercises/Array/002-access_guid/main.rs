
//Write a Rust program to create an array of integers with size 9 and initialize it with any value. Access and print the element at index 5.


fn main () {

    
    //define an array with size of 9
    let arr: [i32; 9] = [1,2,3,4,5,6,7,8,9];
    
    
    //access and print
    println!("element of index 7 is: {:?}", arr[5]);
}