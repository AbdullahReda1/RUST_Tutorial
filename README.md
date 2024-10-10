# RUST Tutorial

![RUST](https://rustacean.net/assets/rustacean-flat-happy.svg)

## **Rust :**

stands out among programming languages due to several key features that address common problems in systems programming, such as memory safety, concurrency, and performance, while also maintaining flexibility for a variety of use cases. Here’s what makes Rust special and why you might choose it over other languages:

### 1. **Memory Safety Without Garbage Collection**

* **Problem** : In languages like C and C++, manual memory management (using `malloc`/`free` or `new`/`delete`) is error-prone and leads to issues like  *null pointer dereferencing* ,  *double frees* , and  *buffer overflows* .
* **Rust’s Solution** : Rust ensures *memory safety* at compile-time through its **ownership** and **borrowing** system, without the need for a garbage collector. This allows for efficient memory management while preventing common bugs related to memory handling.
* **Ownership** : Only one owner per resource.
* **Borrowing** : Other parts of the code can borrow references, but Rust’s rules ensure that no references are dangling or violate memory access.

### 2. **Fearless Concurrency**

* **Problem** : Concurrency bugs, such as race conditions and deadlocks, are notoriously hard to catch in languages like C++ and Java.
* **Rust’s Solution** : Rust introduces *fearless concurrency* by ensuring at compile-time that data races are not possible. Rust’s ownership model extends to threads, meaning that the compiler enforces that data shared between threads is either immutable or properly synchronized.
* The **ownership and borrowing rules** naturally enforce thread safety.

### 3. **High Performance Comparable to C/C++**

* **Problem** : High-level languages (e.g., Python, JavaScript) offer productivity benefits but often have significant performance overheads, especially in low-level systems or performance-critical applications.
* **Rust’s Solution** : Rust offers performance that is comparable to C and C++ because:
* There’s no garbage collection, which reduces runtime overhead.
* It uses zero-cost abstractions, meaning that high-level abstractions are compiled down to efficient machine code without performance penalties.

### 4. **Strong Type System**

* **Problem** : Bugs often arise from type mismatches or unsafe assumptions in many languages.
* **Rust’s Solution** : Rust’s **strong and expressive type system** prevents many common bugs. Its type checker is quite rigorous, and features like  **pattern matching** ,  **enums** , and **result/error handling** enforce more predictable, safe coding practices.
* No need for null pointers, as Rust uses `Option<T>` and `Result<T, E>` types to handle cases where something might be missing or where errors might occur.

### 5. **Safe Abstractions with Zero-Cost**

* **Problem** : High-level abstractions can incur performance overhead in languages like Python or JavaScript.
* **Rust’s Solution** : Rust allows for **safe and high-level abstractions** that have no cost at runtime. You can build data structures or systems with the same safety and abstraction as in high-level languages without incurring performance penalties.

### 6. **Tooling and Ecosystem**

* **Cargo** : Rust has an excellent build system and package manager called  **Cargo** . It simplifies dependency management, project setup, and building projects, making development more streamlined.
* **Rustfmt** and  **Clippy** : These tools help with code formatting and linting, ensuring consistent and idiomatic code.
* **Documentation** : Rust has first-class documentation tools built into the language (`cargo doc`), and the community is known for its rich, well-maintained documentation.

### 7. **Active and Welcoming Community**

* **Problem** : Learning low-level languages can often be daunting for newcomers due to complex tooling and error-prone practices.
* **Rust’s Solution** : Rust has a reputation for a friendly and supportive community. The **Rust compiler** provides extremely helpful error messages that guide you toward solving issues. The language’s official book and other resources are also beginner-friendly.

## Why Choose Rust Over Other Languages?

* **Rust vs. C/C++** :
  - **Memory Safety** : Rust offers safety guarantees that C/C++ lack, preventing many classes of bugs at compile time.
  - **Concurrency** : Rust’s strict guarantees help prevent data races, which are common in C++ multi-threaded code.
  - **Tooling** : Cargo is easier to use compared to the build systems commonly used in C++ (like Make, CMake).
* **Rust vs. Go** :
  - **Memory Management** : Go uses garbage collection, which can introduce pauses in execution. Rust has no such pauses, making it ideal for low-latency, real-time systems.
  - **Control** : Rust allows for more fine-grained control over memory and system resources, which is crucial in performance-critical applications.
  - **Generics and Type System** : Rust has a more powerful type system than Go, with features like traits and algebraic data types (enums).
* **Rust vs. Python/JavaScript** :
  - **Performance** : Rust is significantly faster due to its compiled nature and lack of garbage collection. Python and JavaScript are interpreted and rely on high-level abstractions, which incur performance costs.
  - **Memory Safety** : Python and JavaScript don’t provide the same level of memory safety guarantees as Rust.
  - **Concurrency** : Rust handles concurrency much more efficiently than these high-level languages.

### When to Use Rust?

* **Systems Programming** : When building operating systems, embedded systems, or other performance-critical software (like network services).
* **Web Assembly** : Rust has strong support for WebAssembly (Wasm), making it great for high-performance web applications.
* **Concurrency** : When you need safe multi-threaded or parallel execution.
* **Game Development** : For performance-critical game engines or simulations.
* **CLI Tools** : Rust is excellent for building fast, memory-efficient command-line tools.

## **Notes:**

### 1. **Ownership and Borrowing Rules**

**Ownership** and **borrowing** are central concepts in Rust that help manage memory safely without needing a garbage collector. They prevent dangling pointers, double-free errors, and data races at compile time.

#### **Ownership** :

* Every value in Rust has a single "owner" at any given time.
* When the owner goes out of scope, Rust automatically deallocates the memory.

Rules of ownership:

* **One owner** : Each piece of data can only have one owner at a time.
* **Move semantics** : When ownership is transferred (moved) to another variable, the original variable becomes invalid, preventing use-after-free errors.

Example:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of s1 is moved to s2.    // println!("{}", s1); // Error: s1 is no longer valid.
    println!("{}", s2); // Only s2 can be used.
}
```

Here, `s1`'s data is moved to `s2`. After the move, `s1` is no longer valid, and the Rust compiler ensures that accessing it would result in a compile-time error.

#### **Borrowing** :

* Instead of transferring ownership, you can "borrow" a reference to data, either mutably or immutably.
* **Mutable references** : Only one mutable reference to a value at a time.
* **Immutable references** : You can have multiple immutable references, but no mutable references are allowed when immutable ones exist.

Example of borrowing:

```rust
fn main() {
    let mut s = String::from("hello");
    borrow_mut(&mut s);
    println!("{}", s);
}

fn borrow_mut(s: &mut String) {
    s.push_str(", world");
}
```

Here, the function `borrow_mut` borrows a **mutable reference** to `s` and modifies it. Rust ensures that no other part of the code can access `s` while it's being mutated.

### 2. **No `nullptr` (Null Pointer)**

Rust does not allow null pointers by default. Instead of `null`, Rust uses two types to handle cases where a value might be absent:

* **`Option<T>`** : Used when there may or may not be a value. It's similar to nullable types in other languages, but it's explicit.
* `Option<T>` is either `Some(T)` (contains a value) or `None` (contains no value).

Example:

```rust
fn get_value() -> Option<i32> {
    Some(5)
}

fn main() {
    if let Some(x) = get_value() {
        println!("Value: {}", x);
    } else {
        println!("No value");
    }
}
```

This makes handling the absence of values explicit and avoids the risk of dereferencing a null pointer (which would cause a crash in C/C++).

* **`Result<T, E>`** : Used for error handling. It either contains a value (`Ok(T)`) or an error (`Err(E)`).

### 3. **Safety in Rust**

When we talk about Rust being "safe," we mean it guarantees memory safety, thread safety, and prevents certain kinds of bugs  **without needing runtime checks** . Rust’s type system and borrowing rules ensure:

* **Memory Safety** : No buffer overflows, no null pointer dereferencing, and no use-after-free errors.
* **Concurrency Safety** : No data races because of the way ownership and borrowing rules are enforced in multithreaded code.

This is what makes Rust "safe" compared to languages like C/C++, where such bugs are common.

### 4. **How Cargo Doc Compares to Doxygen**

**Cargo Doc** is similar to  **Doxygen** , but it is native to Rust and integrated directly with Rust's tooling. Here's how it works:

* Rust has a built-in way of documenting code through comments like:
  ```rust
  /// This is a documented function.
  fn my_function() {
      // code
  }
  ```
* Running `cargo doc` automatically generates documentation based on these comments.
* The documentation is rendered in HTML and hosted on local servers for easy access.

In contrast, **Doxygen** is a third-party tool used in languages like C++ to generate documentation. While Doxygen is more flexible (supporting multiple languages), Cargo Doc is streamlined for Rust and its ecosystem.

### 5. **No Garbage Collection but Better Than Go**

Rust doesn’t use a garbage collector (GC), whereas **Go** relies on garbage collection to manage memory.

#### **Why Rust is Better for Some Use Cases** :

* **No GC pauses** : Garbage collectors can introduce unpredictable pauses, which is bad for real-time or low-latency applications. Rust avoids this by managing memory at compile time via its ownership system, so there's no need for a runtime GC.
* **Finer control** : Rust provides manual control over memory allocation and deallocation, which is useful in systems programming and embedded environments where every bit of performance matters.

This makes Rust a better choice for performance-critical and low-latency systems where Go’s garbage collection might introduce undesirable delays.

### 6. **Type System in Rust**

Rust's type system is **static** and  **strongly typed** . It enforces types at compile-time, ensuring that your program has no type-related errors at runtime. Key elements of the type system include:

* **Generics** : Allows writing flexible and reusable code. Generics are type parameters that can be used to define data structures or functions that work for multiple types.

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

* **Traits** : Similar to interfaces in other languages. They define shared behavior for types.
* **Enums and Pattern Matching** : Rust’s enums allow you to define types that can take different forms. Pattern matching is a powerful feature that works with enums and other types.

Example:

```rust
enum Pet {
    Dog(String),
    Cat(String),
}

fn describe(pet: Pet) {
    match pet {
        Pet::Dog(name) => println!("It's a dog named {}", name),
        Pet::Cat(name) => println!("It's a cat named {}", name),
    }
}
```

### 7. **Memory Process Visualization: Example**

In the example code you shared:

```rust
fn main() {
    let mut data = vec![1, 2, 3];

    // Ownership prevents memory safety issues.
    let handle = std::thread::spawn(move || {
        data.push(4); // `data` is moved here, ensuring no data race.
        println!("{:?}", data);
    });

    handle.join().unwrap();
}
```

Here's how it works in memory:

1. **Ownership Transfer** :

* `data` is initially owned by `main`. It's stored on the heap (since it's a `Vec`).
* When you call `std::thread::spawn(move || { ... })`, the `move` keyword transfers ownership of `data` to the closure running in the new thread.

2. **Concurrency** :

* `data.push(4)` modifies the vector, but since ownership of `data` has been moved, there’s no way the main thread can access it while the spawned thread is running.
* This prevents data races because the main thread cannot access `data` while the spawned thread is using it.

3. **Memory Management** :

* Rust’s ownership system ensures that `data` is cleaned up (deallocated) after the thread finishes. Since the main thread no longer has access to `data`, Rust knows when the vector is no longer in use and can free its memory safely.

4. **Join** : The `handle.join().unwrap();` ensures that the main thread waits for the spawned thread to finish execution before continuing. This guarantees that `data` is not dropped prematurely.

### What Ownership Is:

* **Ownership** refers to the idea that exactly one part of the program has control over any piece of data at a time.
* When an owner goes out of scope, Rust automatically deallocates the memory associated with that data.
* Ownership moves from one part of the program to another when values are passed around (e.g., moved into functions or between variables).

### What Ownership Is NOT:

* **Not an address** : The ownership system doesn’t directly represent memory addresses, although behind the scenes, the value is stored in memory at some address.
* **Not an ID** : It's not an ID like a unique identifier in the program. It’s simply a rule enforced by the compiler to track where the value is used and ensure no two places in the code simultaneously own the same data.

### Ownership and Addresses:

* The actual data still exists at some memory address, but **ownership** tracks who is responsible for that address, ensuring there is no "dangling pointer" (a reference to an invalid memory location).
* In languages like C or C++, you might manually manage pointers and memory. Rust does this **safely and automatically** through its ownership system, without needing the programmer to track addresses manually.

## What is Ownership in Rust?

**Ownership in Rust** is a **compile-time rule** that the Rust compiler uses to manage memory and enforce safety. It’s not something physical like a memory address or ID, but rather a **logical construct** that dictates which part of your code is allowed to access or modify a particular piece of data at any given time. This is enforced by the compiler, not by something physical in the hardware or memory.

### How the Compiler Enforces Ownership

The **ownership model** in Rust is a set of rules that the **compiler checks** at compile time. When you write Rust code, the compiler performs several checks on the variables and data, ensuring that the rules of ownership are followed. These checks help manage memory without the need for a garbage collector or manual memory management.

Here’s how the compiler enforces ownership:

#### 1. **Ownership Rules**

* **Each value in Rust has a variable that owns it.**
  * For example, if you have a `String` in a variable, that variable is the "owner" of the string’s heap-allocated memory.
* **There can only be one owner at a time.**
  * If you try to have multiple owners (like in the case of copying or assigning the value), the compiler ensures that one "owner" is invalidated when the other one takes control.
* **When the owner goes out of scope, the memory is automatically deallocated.**
  * This ensures that memory is cleaned up without the need for manual memory management.

#### 2. **Move Semantics (Transfer of Ownership)**

* When you **assign** a value from one variable to another, the ownership of the value is  **moved** .
* After a move, the original variable becomes **invalid** and cannot be used again. If you try to use it, the Rust compiler throws an error, preventing you from accidentally accessing invalid memory.

Example:

```rust
let s1 = String::from("hello");
let s2 = s1; // Ownership is moved to s2
// s1 is now invalid, and trying to use it will result in a compile-time error
// println!("{}", s1); // Error: s1 has been moved
println!("{}", s2); // This works fine because s2 now owns the string
```

#### 3. **Borrowing and References (Temporary Access without Ownership Transfer)**

* Instead of moving ownership, you can **borrow** a reference to data.
* Borrowing allows another part of the program to temporarily use the data without taking ownership.
* Borrowing can be **immutable** (many parts of the program can borrow the data without changing it) or **mutable** (only one part can borrow it and modify it at a time).

Example:

```rust
let s = String::from("hello");
borrow_data(&s); // Borrowing an immutable reference, so ownership is not moved
println!("{}", s); // s is still valid after borrowing

fn borrow_data(s: &String) {
    println!("{}", s); // Temporary access to the data without moving ownership
}
```

#### 4. **Lifetimes (Scopes of Borrowing)**

* Rust enforces **lifetimes** to track how long data is valid, ensuring references are only used while their associated data is still in scope. This prevents dangling references or use-after-free errors.

### What **Exactly** is Ownership?

Ownership is a **static analysis** mechanism—meaning that Rust checks all of this at compile time, not at runtime. It’s like a set of **rules** that the compiler applies to your code:

1. **Single Owner** : A value can only have one owner at any point in time. The compiler tracks which part of the code "owns" a value.
2. **Transfer of Ownership** : Ownership can be transferred from one variable to another (e.g., during function calls or assignment), but once transferred, the original owner is no longer valid.
3. **Automatic Deallocation** : Once the owner goes out of scope, Rust automatically deallocates the memory associated with the value. There is no need for a manual `free` or garbage collector.
4. **Borrowing and Access Control** : Ownership can be temporarily shared using references, which allows safe and controlled access to the data without transferring ownership.

### Why is Ownership Conceptual, Not Physical?

Ownership is **conceptual** because it’s  **purely a set of rules enforced by the Rust compiler at compile time** , not something that exists physically in memory or the CPU. It’s a tool to  **reason about data and memory access** .

* **Ownership does not exist in the physical machine** : The hardware only knows about memory addresses, values, and instructions. It has no concept of ownership. Ownership is an abstraction that exists only in the **source code level** and is enforced by the Rust compiler.
* **Ownership tracking happens at compile time** : The Rust compiler inserts no extra runtime code or memory tracking like a garbage collector. Instead, it checks at compile time that ownership rules are followed, preventing memory errors before the program even runs.

### How to Define Ownership

If we were to define **ownership** formally in Rust, it could be described as:

**Ownership** is a compile-time-enforced rule that ensures each piece of data in a Rust program has exactly one owner at a time, controls when data can be accessed or modified, and manages memory deallocation automatically when the owner goes out of scope. This concept guarantees memory safety without needing garbage collection or manual memory management.

## **Structure of a Rust Project**

```plaintext
hello_cargo/
├── Cargo.lock
├── Cargo.toml
├── src/
│   └── main.rs
└── target/
    ├── debug/
    └── release/
```

#### 1. **`Cargo.toml`**

* **Purpose** : The configuration file for your Rust project, similar to `package.json` in JavaScript or `pyproject.toml` in Python.
* **Contents** : It defines your project’s dependencies, metadata (like the project name, version), and build configurations.
* **Functionality** :
* **Metadata** : Project name, version, authors, etc.
* **Dependencies** : Defines the external libraries (crates) your project needs.
* **Build Profiles** : Defines how different build modes (`dev` vs. `release`) should be configured.
  * **Example **:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
# List of dependencies will be here
```

#### 2. **`Cargo.lock`**

* **Purpose** : Locks the exact versions of dependencies used in your project.
* **Contents** : Automatically generated and maintained by Cargo.
* **Functionality** :
* Ensures that every build is reproducible by locking the exact versions of libraries.
* You don’t edit this manually—Cargo manages it for you.
* **Generated when you run `cargo build`** or any command that downloads dependencies.

#### 3. **`src/` Directory**

* **Purpose** : Contains all the source code for your project.
* **Functionality** :
* By convention, Cargo looks for a `main.rs` file here to run the project.
* If you’re writing a library, you’ll have `lib.rs` instead of `main.rs`.
* The directory can be extended with other source files and modules as your project grows.
* **Key Files** :

  * **`main.rs`** :

    * The entry point of your application if it's a binary crate.
    * This file contains the `main()` function, which is the starting point of your Rust program.
    * Example:

    ```rust
    fn main() {
    println!("Hello, world!");
    }
    ```
* **`lib.rs`** :

  * This would be here if your project is a library crate. For binary projects like yours, the focus is on `main.rs`.

#### 4. **`target/` Directory**

* **Purpose** : Stores all the compiled outputs of your project.
* **Functionality** :
* Stores different build artifacts based on the profile (debug or release) and compiler settings.
* This directory is created after you run commands like `cargo build` or `cargo run`.
* It is **not checked into version control** (e.g., Git) by default.
* **Key Subdirectories** :
* **`debug/`** :
  * Contains the output of the project when you run it in **debug mode** (which is the default when you run `cargo build` or `cargo run`).
  * Debug builds prioritize compilation speed and contain extra debugging information (symbols, etc.).
  * Example: `hello_cargo.exe` for Windows or `hello_cargo` binary on Linux/macOS.
* **`release/`** :
  * Contains the optimized build output when you compile with `cargo build --release`.
  * Release builds are optimized for performance and do not contain debugging symbols.
  * Use this when you're ready to distribute or deploy the application.
* Other directories/files in `target/`:
  * **`deps/`** : Contains compiled versions of dependencies used by your project.
  * **`incremental/`** : Tracks incremental compilation information to make rebuilds faster.
  * **`.fingerprint/`** : Stores metadata to track which files have changed between builds.

#### 5. Additional Files/Directories (for larger projects)

* As your project grows, you might see additional directories like:
  * **`tests/`** : Contains integration tests for your project.
  * **`examples/`** : Contains example programs that demonstrate how to use your project.
  * **`benches/`** : Contains benchmarking code.

### Understanding `target/debug/` and `target/release/` Folders

Both `debug/` and `release/` are subdirectories in the `target/` folder. These directories hold the build outputs from **debug** and **release** builds respectively, and the differences between them directly impact things like **binary size** and  **performance** .

#### `target/debug/`

This directory contains the build output when you run commands like `cargo build` or `cargo run`. These builds prioritize **compilation speed** and include  **debugging information** .

* **Purpose** : Quick feedback during development, with extra debugging metadata, larger binary size, and incremental compilation enabled to make rebuilds faster.
* **Key Elements** :
* **Binaries (`.exe` on Windows or ELF binaries on Linux/macOS)** :
  * Contains the compiled executable, e.g., `hello_cargo.exe` (Windows) or just `hello_cargo` (Linux/macOS).
  * Larger size because it includes debugging symbols, useful for tools like debuggers and profilers.
* **`.pdb` Files (Windows)** :
  * Program database files contain debug information for Windows binaries.
* **`.d` Files** :
  * Dependency files, generated during the build process. These track file dependencies to support incremental compilation, ensuring Cargo only recompiles what has changed.
* **`.fingerprint/`** :
  * Stores metadata related to the build to track whether changes have been made, which helps with incremental compilation.
* **`deps/`** :
  * Contains the compiled versions of all the project’s dependencies (e.g., external crates). This ensures the project can link against the compiled libraries efficiently without recompiling them every time.
* **`incremental/`** :
  * Holds information used for  **incremental compilation** . Rust can reuse previous compilation results, reducing the need to recompile the entire project after small changes.
  * Files inside this directory include various object files (`.o`), dependency graphs (`dep-graph.bin`), and query caches (`query-cache.bin`), which help Rust rebuild only what’s necessary.
* **`.cargo-lock`** :
  * A lock file indicating the state of Cargo’s build process.

#### `target/release/`

This directory holds the build output when you run `cargo build --release`. Release builds are optimized for  **performance** , but take longer to compile and produce **smaller binaries** compared to debug builds.

* **Purpose** : Final production builds, optimized for execution speed, memory usage, and reduced binary size. Typically used for deployment or distributing the software.
* **Key Elements** (similar to `debug/`, but with some distinctions):
  * **Binaries** :
  * Smaller size compared to debug builds because all  **debugging symbols are stripped** , and the  **code is optimized** .
  * **`.pdb` Files (Windows)** :
  * Even in release mode, `.pdb` files might be generated, but they contain minimal information unless explicitly configured to include more debug data.
  * **`.d` Files and `.fingerprint/`** :
  * Like in debug mode, these files track dependencies and metadata but are tuned for the release build.
  * **`deps/`** :
  * Contains optimized versions of the dependencies used in the release build.
  * **`incremental/`** :
  * While present, Rust doesn’t usually perform incremental compilation in release mode to ensure the final build is fully optimized.

---

### Why This Separation Matters

The structured organization in `target/` ensures Cargo and the Rust compiler can efficiently manage:

1. **Incremental Compilation** :

* By tracking file dependencies and generating `.d` files and other metadata, Cargo can selectively rebuild only parts of the project that have changed, saving time during development. This is why files like `dep-graph.bin`, `query-cache.bin`, and `.fingerprint/` are crucial.

2. **Optimized Binaries** :

* In release mode, binaries are stripped of debug symbols and optimized. This reduces the size and improves performance, making the final program faster and lighter for distribution.

3. **Debugging and Profiling** :

* The debug build includes extra information (e.g., `.pdb` files, symbols), making it easier to debug and profile. The separation of these files in the `debug/` folder allows you to keep the **debugging process** independent from the  **final optimized build** .

4. **Descriptive File Names and Layout** :

* The structured file and folder names (like `bin-hello_cargo`, `.fingerprint/`, and `.cargo-lock`) reflect their specific roles, making it easier to understand how each part contributes to the build process.

5. **Performance** :

* The separation of build profiles (`debug/` and `release/`) allows for performance optimizations when needed. For development, you prioritize **speed** by using the debug mode, and for deployment, you switch to release mode to optimize the **binary size** and  **runtime performance** .

6. **File Structure Transparency** :

* Files like `.json`, `.pdb`, `.lock`, `.d`, and others clearly communicate their purpose (e.g., tracking dependencies, debugging information, etc.), making the entire build process more transparent and easier to manage.
