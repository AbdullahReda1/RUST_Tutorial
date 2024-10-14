/*
    1. The `io` library comes from the standard library known as `std` library 
       provides the ability to accept user input with a `use` statement.
    2. the line `use rand::Rng;`. The `Rng` `trait` defines methods that random number generators implement, 
       and this trait must be in scope to use those methods.
    3. The `Ordering` type is another enum and has the variants `Less`, `Greater`, and `Equal`.
*/
use std::{io, ops::AddAssign, cmp::Ordering};
use rand::Rng;

fn main(){
    println!("Guess the number!");

    /*
        1. `thread_rng` Initializing the random numbers generator on the first call on each thread.
        2. `gen_range` Generate a random value in the given range, it's optimized for the case that
           only a single sample is made from the given range.
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Printing Values with println! "Placeholders".
    ////println!("The secret number is: {secret_number}");

    // Initialize a counter for the number of attempts.
    let mut attempts: u32 = 0;

    // The `loop` keyword creates an infinite loop.
    loop {
        println!("Please input your guess.");

        /*
            1. Use the `let` statement to create the variable.
            2. To make a variable mutable, we add `mut` before the variable name.
            3. The equal sign (=) tells Rust we want to bind something to the variable now.
            4. `new` function creates a new value of some kind in this case empty string.
        */
        let mut guess: String = String::new();

        /*
            1. Call the `stdin` function from the `io` module, which will allow to handle user input.
            2. The `&` indicates that this argument is a reference, which gives you a way to let multiple
            parts of your code access one piece of data without needing to copy that data into memory multiple times.
            3. The full job of `read_line` is to take whatever the user types into standard input and append 
            that into a string (without overwriting its contents), so that string passed as an argument.
            4. Handling Potential Failure with Result: An instance of Result has an `expect` method that called.
            If this instance of `Result` is an `Err` value, expect will cause the program to crash and display
            the message that it passed as an argument to `expect`.
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // "Shadowing" lets us reuse the `guess` variable name.
        // The `trim` method on a String instance will eliminate any whitespace at the beginning and end.
        // The `parse` method on strings converts a string to another type.
        // The colon (:) after guess tells Rust to annotate the variable’s type.
        // The `u32` seen here is an unsigned, 32-bit integer.
        /*
            1. Switch from an expect call to a match expression to move from crashing on an error to handling the error.
            2. `parse` method returns a Result type as an enum that has the variants `Ok` and `Err`.
            3. `Ok` value that contains the resultant number. That `Ok` value will match the first arm’s pattern,
               and the match expression will just return the `num` value that parse produced and put inside the `Ok` value.
            4. If `parse` is not able to turn the string into a number, it will return an `Err` value 
               that contains more information about the error which is the second `match` arm.
            5. The underscore '_' is a catchall value that match all `Err` values, no matter what information they have inside them.
            6. Program will execute the second arm’s code `continue` which tells the program to go to the next iteration of 
               the loop and ask for another guess. So, effectively, the program ignores all errors that parse might encounter!
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Increment the attempts counter.
        attempts.add_assign(1);

        // Printing Values with println! "Placeholders".
        println!("You guessed: {}", guess);

        /*
            1. The match Control Flow Construct: match expression to decide what to do next based 
            on which variant of Ordering was returned from the call to cmp with the values in guess and secret_number.
            2. The cmp method compares two values and can be called on anything that can be compared.
            3. `break` line after You win! makes the program exit the loop when the user guesses the secret number correctly.
               Exiting the loop also means exiting the program, because the loop is the last part of `main`.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win! It took you {} attempts.", attempts);
                break;
            }
        }
    }
}