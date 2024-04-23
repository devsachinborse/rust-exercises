fn main() {
    let num1 = 23;
    let num2 = 98;

    //addition
    let addition_result = num1 + num2;
    println!("Addition result: {}", addition_result);

    //substraction operation
    let substraction_res = num1 - num2;
    println!("Substraction result : {}", substraction_res);

    //Multiplication
    let multiplication_res = num1 * num2;
    println!("Multiplication result: {}", multiplication_res);

    // Division operation
    // Check if the divisor is not zero before performing division
    if num2 != 0 {
        let division_res = num1 / num2;
        println!("division_res: {}", division_res);
    } else {
        println!("can not divide by 0");
    }
}
