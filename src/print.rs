pub fn run() {
    //print to console
    println!("Hello from the public function");

    //Basic formatting
    println!("{} is from {}", "Brad", "Mass");

    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    //Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "john",
        activity = "baseball"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}
