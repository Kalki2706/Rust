fn main() {
    // Calling panic! macro to see error
    // panic!("crash and burn!");

    // Using panic! backtrace with calling value in vector beyond it's indexes. Read more about panic errors in Chapter 9.
    let v = vec![1, 2, 3];
    v[99];
}
