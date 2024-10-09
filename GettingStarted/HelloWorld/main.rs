fn main() {
    // `println!` calls a Rust macro. If it had called a function instead, it would be entered as println (without the !).
    println!("Hello, world!");
}

/*
    Just compiling with `rustc` is fine for simple programs, but as your project grows, 
    you’ll want to manage all the options and make it easy to share your code. 
    Next, we’ll introduce you to the Cargo tool, which will help you write real-world Rust programs.
*/