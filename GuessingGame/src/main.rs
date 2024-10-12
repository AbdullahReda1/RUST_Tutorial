/*
    The `io` library comes from the standard library known as `std` library 
    provides the ability to accept user input with a `use` statement
*/
use std::io;

fn main(){
    println!("Guess the number!");
    println!("Please input your guess.");

    /*
        Use the `let` statement to create the variable.
        To make a variable mutable, we add `mut` before the variable name.
        The equal sign (=) tells Rust we want to bind something to the variable now.
        `new` function creates a new value of some kind in this case empty string.
    */
    let mut guess= String::new();

    /*
        Call the `stdin` function from the `io` module, which will allow to handle user input.
        The `&` indicates that this argument is a reference, which gives you a way to let multiple
        parts of your code access one piece of data //!without needing to copy that data into memory multiple times.
        The full job of `read_line` is to take whatever the user types into standard input and append 
        that into a string (without overwriting its contents), so that string passed as an argument.
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    /* Printing Values with println! //^ "Placeholders" */
    println!("You guessed: {}", guess);
}