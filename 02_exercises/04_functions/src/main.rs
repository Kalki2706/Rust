// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

fn f(x: i32) {
    println!("{x}");
}

// Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

fn fun(x: i32) -> i32 {
    x + 1
}

fn main() {
    f(0);

    println!(
        "{}",
        fun({
            let y = 1;
            y + 1
        })
    );
}
