//Write a Rust function that calculates the circumference of a circle using the constant PI and a given diameter

const PI:f64 = 3.14;

fn cal_circumference (dia: f64) -> f64{
    let radius = dia/2.0;
    let circumference = 2.0 * PI * radius;
    return circumference
}

fn main () {
   
   let dia = 20.0;
   let circumference = cal_circumference(dia);
   println!("The circumference of the circle with dia {} is {:.2}", dia, circumference);

}