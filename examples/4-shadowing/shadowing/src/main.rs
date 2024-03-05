// This is shadowing because we are reusing the variable name health to store a different type of value.

fn main() {
    let mut height = 190;
    height = height - 20;
    println!("Height: {}", height);
    let result = if height < 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };

    println!("Result: {}", result);

    // First declared as a address to a string
    let health = if height < 180 {"good"} else {"unknown"};
    println!("Health: {}", health);
    
    // Shadowing to a different type; changed from string to boolean
    let health = if height < 180 {true} else {false};

}
