fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    another_function();
}

fn another_function() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");
    let x = 7;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // <---- Difference between shadowing and mut ---->
    let spaces = "    ";
    let spaces = spaces.len();

    println!("{spaces}");

    // <---- Do `cargo check` in terminal by uncommenting the following code ---->

    // let mut spaces = "   ";
    // spaces = spaces.len();

    // println!("{spaces}");
}
