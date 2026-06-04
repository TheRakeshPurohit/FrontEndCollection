//! # Lesson 001: Variables & `println!`
//!
//! Topics:
//! - `let` bindings and immutability by default
//! - Type inference
//! - The `println!` macro and inline format args (`{name}` since Rust 2021)
//!
//! Run with: `cargo run --bin 001-variables`

fn main() {
    println!("=== Lesson 001: Variables & println! ===\n");

    // ----- 1. Basic variable binding -----
    // `let` introduces a new variable. No type annotation needed — Rust infers it.
    let name = "Rust";              // inferred type: &str  (a string slice)
    let year = 2026;                // inferred type: i32   (default integer)
    let pi = 3.14;                  // inferred type: f64   (default float)
    let is_fun = true;              // inferred type: bool

    // ----- 2. Printing with println! -----
    // `println!` is a MACRO (note the `!`). Macros are expanded at compile time.
    // `{}` is a placeholder; arguments after the format string fill them in order.
    println!("Hello, {}!", name);

    // Since Rust 2021, you can put the variable name directly inside `{}`.
    // This is the modern, preferred style for simple names.
    println!("The year is {year}.");
    println!("Pi is approximately {pi}.");
    println!("Is Rust fun? {is_fun}.");

    // ----- 3. Multiple values in one call -----
    println!("{name} {year} {pi} {is_fun}");

    // ----- 4. Variables are IMMUTABLE by default -----
    // This would fail to compile:
    //     let x = 5;
    //     x = 6;   // error[E0384]: cannot assign twice to immutable variable
    // To allow mutation, use `let mut` (covered in Lesson 002).

    // ----- 5. Debug vs Display printing (preview of Lesson 020 — Traits) -----
    // {}    uses the `Display` trait — for human-friendly output.
    // {:?}  uses the `Debug` trait   — for developer-friendly output of any
    //                                   compound type (tuples, arrays, etc.).
    let nums = [1, 2, 3];
    // println!("{}", nums);          // ❌ arrays don't implement Display
    println!("{:?}", nums);           // ✅ [1, 2, 3]
    println!("{nums:?}");             // ✅ same thing, inline form
}
