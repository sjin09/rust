fn main() {
    // Variables and Mutability
    // let mut x = 5; // variable is mutable
    // println!("The value of x is : {}", x);
    // x = 6;
    // println!("The value of x is : {}", x);

    // Shadowing
    // Comparing the Guess to the Secret Number” on page 23, 
    // you can declare a new variable with the same name as a previous variable, 
    // and the new variable shadows the previous variable. 
    // Rustaceans say that the first variable is shadowed by the second, 
    // which means that the second variable’s value is what appears when the variable is used.
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x); // 12

    // Data Types
    // Keep in mind that Rust is a statically typed language, 
    // which means that it must know the types of all variables at compile time.

    // functions 
    // Rust cost uses snake case as the conventional style for functions
    // and variable names. In snake case, all letters are lowercase and underscores
    // spearate words.

    // Statements and expressions in function bodies
    // When do you use expressions isntead of statements?
    // let x = 5;
    // let y = {
    //     let x = 3;
    //     x + 1
    // }; // evaluates to 4
    // If you add a semicolon to the end of an expresion, you turn it into a statement

    // functions with return values

    // Control flow
    // Rust will not automatically try to convert non-Boolean types to Boolean.
    // You must be explicit and always provide if with a Boolean and its condition

    // Using if in a let Statement
    // Because if is an expression, we can use it on the right side of a let statement
    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("The value of number is: {}", number); // 5


    // Repetition with Loops
    /// Repeating code with loop
    
    // loop {
    //     println!("again!");
    // }

    // Returning values from loops
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // }
    // println!("The result is {}", result);

    // Condition loops with while
    // let mut number = 3;
    // while number != 0 {
    //     println!("{}!", number);
    //     number = number - 1;
    // }
    // println!("LIFTOFF!!!");

    // Looping through a collection with for
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index = index + 1;
    // }

    // let a = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
