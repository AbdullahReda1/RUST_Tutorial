fn main() {
    println!("Hello, world!");
}
/*
    We can create a project using `cargo new`.
    We can build a project using `cargo build`.
    We can build and run a project in one step using `cargo run`.
    We can build a project without producing a binary to check for errors using `cargo check`.
    We can compile a project with optimizations using `cargo build --release`.
*/
/*
*PS E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo> //! cargo new hello_cargo
   Compiling hello_cargo v0.1.0 (E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in //^ 4.20s
*PS E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo\hello_cargo> //! cargo build
   Compiling hello_cargo v0.1.0 (E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in //^ 0.99s
*PS E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo\hello_cargo> //! cargo run  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in //^ 0.01s
     Running `target\debug\hello_cargo.exe`
Hello, world!
*PS E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo\hello_cargo> //! cargo check
    Checking hello_cargo v0.1.0 (E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in //^ 0.23s
*PS E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo\hello_cargo> //! cargo build --release
   Compiling hello_cargo v0.1.0 (E:\ITI ES\RUST\RUST_Tutorial\GettingStarted\HelloCargo\hello_cargo)
    Finished `release` profile [optimized] target(s) in //^ 0.59s
*/