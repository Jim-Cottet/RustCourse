pub fn run() {

    // End of Print
    println!("! ---------- Print.rs ----------- !");
    
    //print to console
    println!("From the print file");
    
    // Basic formating
    println!("Number: {}", 1);

    //Position argument
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code" );

    //Named Argumenst
    println!(
        "{name} likes to play {activity}",
        name = "Johne",
        activity = "Baseball" 
    );

    // Placeholder traits
    println!("Binary {:b} Hex {:x} Octo {:o}", 10, 10, 10);

    // Placeholder fot debug trait
    println!("{:?}", (12, true, "Hello"));

    
    
}