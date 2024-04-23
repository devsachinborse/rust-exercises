
fn main () {
    
    
    println!("Enter the number: ");
    
    //create mutable string variable to store user input
    let mut input = String::new();
    
    //Read the user input from the standard input
    std::io::stdin().read_line(&mut input).expect("Failed to real the value");
    
    //convert the user input to an integer
    let number:i32 = input.trim().parse().expect("Please enter a value");
    
    
    //check even or odd
    if number%2 == 0 {
        println!("The number {} is even", number);
    } else {
        println!("The number {} is odd", number);
    }
    
    
    
}