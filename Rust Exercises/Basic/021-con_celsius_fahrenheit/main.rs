
//Write a Rust program that creates an integer variable temp and assign it a temperature value in Celsius. Convert it to Fahrenheit and print the result.


fn celsius_to_fahrenheit(celsius: i32) -> f64 {
    let fahrenheit = (celsius as f64 * 9.0 / 5.0) + 32.0;

    fahrenheit
}

fn main () {

    let temp_celsius :i32 = 40;

    let temp_fahrenheit: f64 = celsius_to_fahrenheit(temp_celsius);

    println!("Temprature in celsius : {:.2} °C", temp_celsius );
    println!("Temprature in fahrenheit : {:.2} °F", temp_fahrenheit );

}