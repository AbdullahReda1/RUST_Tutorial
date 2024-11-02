fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
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