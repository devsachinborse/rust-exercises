
fn smallest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() <= s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    
    let s1 = "Iron Man";
    let s2 = "Thor";
    
    let smallet = smallest_string(s1,s2);
    println!("The smallest string is: {}", smallet);
}