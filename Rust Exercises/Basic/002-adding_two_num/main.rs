use std::io;


fn main() {

    println!("Enter the first number");
    let mut input_num1 = String::new();
    io::stdin().read_line(&mut input_num1).expect("faild to read input");
    let num1:f64 = input_num1.trim().parse().expect("please enter the value");


    println!("Enter the second number");
    let mut input_num2 = String::new();
    io::stdin().read_line(&mut input_num2).expect("faild to read input2");
    let num2 :f64 = input_num2.trim().parse().expect("Please enter the value");


    //sum
    let sum = num1 + num2;
    println!("The sum of {:?} and  {:?} is: {:?}", num1,num2,sum);

}