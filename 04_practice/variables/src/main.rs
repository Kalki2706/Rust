fn main() {
    
// BINDING & MUTABILITY
// A variable can be used only if it has been initialized.

    {
    // 1. Fix the error below with least amount of modification to the code
    
        let x: i32 = 5; // Uninitialized but used, ERROR !
        let _y: i32; // Uninitialized but also unused, only a Warning ! 

        assert_eq!(x, 5); // Assert checks two expressions are equal to each other.
        println!("Success!");

    // SOLUTION: Used '_' called as clippy lints in rust to use before uninitialized variable & assigned 5 to variable x to make it LHS == RHS.
    }

    {
    // 2. Fill the blanks in the code to make it compile

        let mut x: i32 = 1;
        x += 2;
        
        assert_eq!(x, 3);
        println!("Success!");

    // SOLUTION: Annotating i32 to x: not necessary, filled mut & x variable, assigned += to x variable
    }

// SCOPE
// A scope is the range within the program for which the item is valid.

    {
    // 3. Fix the error below with least amount of modification

        let x: i32 = 10;
        let y: i32 = 5;

        {
            println!("The value of x is {} and value of y is {}", x, y);
        }

        println!("The value of x is {} and value of y is {}", x, y);

    // SOLUTION: You can't access variable inside of a scope, so I removed this line `let y: i32 = 5;` from inner scope and put it outside of that block to not get error.
    }

    {
    // 4. Fix the error with the use of define_x

        fn define_x() {
            let x: &str = "hello";
            println!("{}, world", x);
        }
        define_x();

    // SOLUTION: called define_x() outside of it's block
    }

// SHADOWING
// You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.

    {
    // 5. Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)

        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12);
        }

        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x); // Prints "42".

    // SOLUTION: Moved assert_eq with each other.
    }

    {
    // 6. Remove a line in the code to make it compile
        let mut _x: i32 = 1;
        _x = 7;
        // Shadowing and re-binding
        _x += 3;
        
        
        let _y: i32 = 4;
        // Shadowing
        let _y: &str = "I can also be bound to text!"; 
        
        println!("Success!");
    }
    {
    // 7. Fix the warning: unused variable: 'x'
        #[allow(unused_variables)]
        let _x: i32 = 1;
        
    // SOLUTION: added #(allow[unused_variables)] and clippy lints so the warning will be not generated.
    }
}
