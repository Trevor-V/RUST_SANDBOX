// Arrays - Fixed list where elemnts are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //reassign a value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //get a single value
    println!("Single Value: {}", numbers[0]);

    //get array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];

    println!("SLice: {:?}", slice);
}
