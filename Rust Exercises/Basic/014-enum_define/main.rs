//Write a Rust program that defines an enum Color with variants representing different colors.

#[derive(Debug)] //add this to auto implement
enum Color {
    Pink,
    Blue,
    White,
}

fn main() {

    // create variabels of type Color using enum varients
    let pink = Color::Pink;
    let blue = Color::Blue;
    let white = Color::White;


    //print
    println!("Red : {:?}", pink);
    println!("Red : {:?}", blue);
    println!("Red : {:?}", white);

}
