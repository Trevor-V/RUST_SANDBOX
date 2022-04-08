/*
Primative Types--
Integers: u8, i8, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in
memory)
floats: f32, f64
Boolean (bool)
Characters (char)
tuples
Arrays
*/

/*
Rust is a statically typed languege, which means that it must know the types of all
variables at compile time, however, the complier can usually infer what type we want to use
based on the value and who we use it.
*/

pub fn run() {
    // default is "i32"
    let x = 1;

    //default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 4545554545;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;

    //get boolean from expression
    let is_greater = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_greater, a1, face))
}
