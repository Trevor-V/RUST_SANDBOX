//Primitive str = Immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length {}", hello.len());

    hello.push('W');

    hello.push_str("orld");

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    println!("Is Empty {}", hello.is_empty());

    println!("Contains 'World': {}", hello.contains("World"));

    println!("Replace: {}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
