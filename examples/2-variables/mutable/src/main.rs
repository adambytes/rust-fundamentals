
fn main() {
    let mut message = String::from("Name: Alfredo, Height: ");
    // Shows me how to clear a mutable variable by truncating it
    message.clear();
    // Shows me how to reassign a mutable variable
    let mut height = 190;
    height = 189;
    println!("{}{}", message, height);

}
