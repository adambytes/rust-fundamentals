use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    // read_line returns a Result type, so we use expect to handle the error
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // use of match expression to pattern match against variable "name"
    // Challenge 1: add more greetings
    // Challenge 2: case insensitive match

    match name.trim().to_lowercase().as_str() {
        "good afternoon" => println!("Good afternoon to you too!"),
        "good morning" => println!("Good morning to you too!"),
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        "whats up" => println!("Not much, you?"),
        _ => println!("I can't find a greeting, good bye."),
    }
}

