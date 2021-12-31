// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure -  Use when you need to modify or own string data 

pub fn run() {

    println!("!---------------- Strings -------------------!");

    //Primitive str :
    let hello = "Hello";

    // String
    let mut growable = String::from("Hello");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push Sring
    hello.push_str("orld !");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Is_empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace());

    // Loop through string by whitespace
    for word in hello.splite_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push("a");
    s.push("b");

    // Assertion testing
    assert_eq!(2, s.len())// pass
    assert_eq!(3, s.len())// dont pass

    
    println!,("{}", hello);

}