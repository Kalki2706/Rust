// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
// fn main() {
    //   let x: fsize = 2.0;

//   println!("{x}");
// }
// ***** The answer is, this program won't compile 'cause there is only f32 & f64. *****


// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.
fn main() {
    let t = ([1; 2], [3; 4]);

    let (a, _b) = t;

    println!("{}", a[0] + t.1[0]); 
}

// The answer is it does compile amd the answer is 4.