
//Write a Rust program to create an array of characters with size 6 and initialize it with the characters 'A', 'B', 'C', 'D', 'E', and 'F'. Update the element at index 3, 5 to 'P' and 'R' resp and print the updated array.

fn main () {
    
    //define array with the size of 6 char
    let mut arr : [char; 6] = ['A', 'B', 'C', 'D', 'E', 'F'];
    
    //update the element at index 3
    arr[3] = 'P';
    arr[5] = 'R';
    
    //print updated array
    println!("update: {:?}", arr);
}