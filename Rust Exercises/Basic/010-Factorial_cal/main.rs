fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    let num = 5; //Example number 
    let result = factorial(num as u64);
    println!("Factorial of {} is {}", num, result);
}
