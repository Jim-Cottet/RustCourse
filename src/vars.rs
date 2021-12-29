pub fn run() {
    
    // In rust 'let' unlike JS, create an unmutable constant
    let name = "Brad";
    // We have to use the mut atttribute to make clear that our variable is mutable
    let mut age = 37;
    // Only the we can change his value
    age = 38;

    println!("My name is {0}, I'm {1}", name, age);

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID)
}