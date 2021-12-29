pub fn run() {
    
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

    // End of Print
    println!("! ---------- End of Print.rs ----------- !");
    
}