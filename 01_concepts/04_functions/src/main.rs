fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let five = five();

    println! {"The return function value is: {five}"};
    // println!("{}", five());

    let add = plus_one(5);

    println!("The value of plus one is: {add}");
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

    // Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:
    /*
    let x = (let y = 6);
    let a = b = 6;
    println!("{a}, {b}");
    */

    // let a = let b = 6;
    // println!("{a}, {b}");
}

// functions with return values
fn five() -> i32 {
    5
}

// another return function
fn plus_one(add: i32) -> i32 {
    add + 1
}
