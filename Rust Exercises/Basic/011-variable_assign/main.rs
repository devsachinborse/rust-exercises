fn main() {
    let p = 10; // Assign a value to p
    let mut _q;  // Declare q without assigning a value
    
    _q = p; // Assign p to q
    // Using p again
    // println!("Value of p: {}", p); // Uncommenting this line will result in a compile error
    println!("Value of q: {}", _q); // Prints the value of q
}
