
//Write a Rust program that defines a struct Person with fields like name and age

struct Person {
    name : String,
    age: i32,
}

fn main() {
    
    let actor = Person {
        //assign values to 'name' and 'age' fields
        name: String::from("Tony stark"),
        age:50,
    }; 
    
    println!("Name: {}" , actor.name);
    println!("age: {}" , actor.age);
}