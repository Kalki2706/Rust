use std::io;

fn main() {
    // <---- Primary Scalar Types ---->

    // <-- Integer types -->
    let number = -50; // i32 by default
    println!("{}", number);

    // <-- Floating types -->
    let x = 2.0; // f64 by default which 64bits, it's double precision

    let y: f32 = 3.0; // f32 which is 32bits, it's single precision

    println!("{x}, {y}");

    // f64 is more precise and roughly the same speed as f32

    // <-- Numeric Operations -->

    // addition
    let _sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {difference}");

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    let truncated = -5 / 3;
    println!("{truncated}");

    // <-- Clippy Lints -->
    // Using clippy lints shows that you will not use this binding `_a`, but it's bad practice if you're gonna use this binding later with macros. Just convert it if you're gonna use it, from `_a` => a.

    let _a = 15; // convert it to `a`
    println!("{_a}");

    // remainder
    let _remainder = 43 % 5; // Remainder is 3

    // <-- Boolean types -->
    let a = true;
    println!("{a}, {_a}"); // a and _a is different

    let _b: bool = false; // with explicit type annotation

    // <-- Character types -->
    let c = 'z';
    println!("{c}");

    let c: char = 'Z'; // with explicit type annotation
    println!("{c}");

    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    // <---- Compound Types ---->

    // <-- Tuple type -->
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // accessing multiple values with indexing by using `.`
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{five_hundred}, {six_point_four}, {one}");

    // Array type
    let array = [1, 2, 7, 5];
    let new_array = array[2];
    println!("{}", new_array);

    let second_array: [i32; 5] = [1, 2, 3, 4, 5]; // will throw an error if you don't put 5 values in array

    println!("{:?}", second_array);

    let third_array = [3; 5]; // will add 3 in all 5 places like this, [3, 3, 3, 3, 3]

    println!("{:?}", third_array);

    another_function();
}

// <---- Array Program ----> 

fn another_function() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}