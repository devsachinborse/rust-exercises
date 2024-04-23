
fn main () {
    
    //initialize variable to start counting
    let mut count = 1;
    
    //start loop
    while count<=20 {
        
        //check if the number is even
        if count%2 == 0 {
            
            //print even number
            println!("number: {}", count);
        }
        
         // Increment count by 1
        count += 1;
    }

}