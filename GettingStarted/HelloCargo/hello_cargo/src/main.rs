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
*PS Path\to\dir> //! cargo new hello_cargo
   Compiling hello_cargo v0.1.0 (Path\to\dir\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in //^ 4.20s
*PS Path\to\dir\hello_cargo> //! cargo build
   Compiling hello_cargo v0.1.0 (Path\to\dir\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in //^ 0.99s
*PS Path\to\dir\hello_cargo> //! cargo run  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in //^ 0.01s
     Running `target\debug\hello_cargo.exe`
Hello, world!
*PS Path\to\dir\hello_cargo> //! cargo check
    Checking hello_cargo v0.1.0 (Path\to\dir\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in //^ 0.23s
*PS Path\to\dir\hello_cargo> //! cargo build --release
   Compiling hello_cargo v0.1.0 (Path\to\dir\hello_cargo)
    Finished `release` profile [optimized] target(s) in //^ 0.59s
*/

/*
* hello_cargo/
* ├── Cargo.lock
* ├── Cargo.toml
* ├── src/
* │   └── main.rs
* └── target/
*     ├── .rustc_info.json
*     ├── CACHEDIR.TAG
*     ├── debug/
*     │   ├── .cargo-lock
*     │   ├── hello_cargo.d
*     │   ├── hello_cargo.exe
*     │   ├── hello_cargo.pdb
*     │   ├── .fingerprint/
*     │   │   ├── hello_cargo-324d85f2ed949151/
*     │   │   │   ├── bin-hello_cargo
*     │   │   │   ├── bin-hello_cargo.json
*     │   │   │   ├── dep-bin-hello_cargo
*     │   │   │   └── invoked.timestamp
*     │   │   └── hello_cargo-c0fe7de074aed428/
*     │   │       ├── bin-hello_cargo
*     │   │       ├── bin-hello_cargo.json
*     │   │       ├── dep-bin-hello_cargo
*     │   │       └── invoked.timestamp
*     │   ├── build/
*     │   ├── deps/
*     │   │   ├── hello_cargo-324d85f2ed949151.d
*     │   │   ├── hello_cargo.d
*     │   │   ├── hello_cargo.exe
*     │   │   ├── hello_cargo.pdb
*     │   │   └── libhello_cargo-324d85f2ed949151.rmeta
*     │   ├── examples/
*     │   └── incremental/
*     │       ├── hello_cargo-1uu4t2cs9z7yw/
*     │       │   ├── s-h0pi2klant-0fvwlc1-2lghz50djdvjc5hvh6tcuozom/
*     │       │   │   ├── dep-graph.bin
*     │       │   │   ├── query-cache.bin
*     │       │   │   └── work-products.bin
*     │       │   └── s-h0pi2klant-0fvwlc1.lock
*     │       └── hello_cargo-1yk13aj5fkw4k/
*     │           ├── s-h0pilljlll-1ojh1do-6vwd9e07mbnm8qlastijw1w01/
*     │           │   ├── 0dvfi3q95go7bu4ljvjw062kd.o
*     │           │   ├── 2scsexmxxnqcrrsdiq6zn1k57.o
*     │           │   ├── 40dip2xfw727j8p2c5tjr9kvw.o
*     │           │   ├── 4oa3aadsft89rls8rrpk08gho.o
*     │           │   ├── bkd5ks1qmwbgwbqwmwqyhog54.o
*     │           │   ├── bnvwq9rvcymfn6sdt2os66m1e.o
*     │           │   ├── dep-graph.bin
*     │           │   ├── query-cache.bin
*     │           │   └── work-products.bin
*     │           └── s-h0pilljlll-1ojh1do.lock
*     └── release/
*         ├── .cargo-lock
*         ├── hello_cargo.d
*         ├── hello_cargo.exe
*         ├── hello_cargo.pdb
*         ├── .fingerprint/
*         │   └── hello_cargo-c0fe7de074aed428/
*         │       ├── bin-hello_cargo
*         │       ├── bin-hello_cargo.json
*         │       ├── dep-bin-hello_cargo
*         │       └── invoked.timestamp
*         ├── build/
*         ├── deps/
*         │   ├── hello_cargo.d
*         │   ├── hello_cargo.exe
*         │   └── hello_cargo.pdb
*         ├── examples/
*         └── incremental/
*/