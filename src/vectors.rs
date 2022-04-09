// Arrays - Fixed list where elemnts are the same data types

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //reassign a value
    numbers[2] = 20;

    //add on to vector
    numbers.push(6);
    numbers.push(7);

    //Pop off the last value
    numbers.pop();

    println!("{:?}", numbers);

    //get a single value
    println!("Single Value: {}", numbers[0]);

    //get Vector length
    println!("Vector length: {}", numbers.len());

    //Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("SLice: {:?}", slice);

    //Loop though vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //Loop & Mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    print!("Numbers Vec {:?}", numbers);
}
