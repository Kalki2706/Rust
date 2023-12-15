fn main() {
    // Calling panic! macro to see error
    // panic!("crash and burn!");

    // Using panic! backtrace with calling value in vector beyond it's indexes.
    let v = vec![1, 2, 3];
    v[99];
}
