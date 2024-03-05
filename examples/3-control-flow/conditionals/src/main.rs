fn main() {
    let proceed = false;
    // Difference from JS is that if statements don't require parentheses
    if proceed {
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 190;
    if height < 180 {
        println!("Tall");
    } else if height > 170 {
        println!("Average");
    } else {
        println!("Short");
    }

}
