fn main() {
    /* //! Variables and Mutability  */
    //let x = 5;    // Immutable variable (can't be changed).
    let mut x = 5;  // mutable variable (can be changed).
    println!("The value of x is: {x}");
    //x = 6;       // Error message "cannot assign twice to immutable variable `x`" because of assign a second value to the immutable variable.
    x = 6;         // Not Error.
    println!("The value of x is: {x}");

    /* //! Constants  */
    /*
        Like immutable variables, constants are values that are bound to a name and are not allowed to change,
        but there are a few differences between constants and variables:
        1. First, you aren’t allowed to use `mut` with constants,
           declare constants using the const keyword instead of the let keyword.
        2. The last difference is that constants may be set only to a constant expression,
           not the result of a value that could only be computed at runtime.
    */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    /* //! Shadowing  */
    
}

/*
If (let x = 5;):
----------------
    error[E0384]: cannot assign twice to immutable variable `x`
    --> src/main.rs:4:5
    |
    2 |     let x = 5;
    |         - first assignment to `x`
    3 |     println!("The value of x is: {x}");
    4 |     x = 6;
    |     ^^^^^ cannot assign twice to immutable variable
    |
    help: consider making this binding mutable
    |
    2 |     let mut x = 5;
    |         +++
*/