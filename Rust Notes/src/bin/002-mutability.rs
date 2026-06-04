//! # Lesson 002: Mutability, Constants, Shadowing
//!
//! Topics:
//! - `let mut` — opt-in mutability
//! - `const` — true compile-time constants
//! - Shadowing — reusing a name with a new `let`
//!
//! Run with: `cargo run --bin 002-mutability`

// ----- Constants can live at module (file) level -----
// - Type annotation is REQUIRED for `const`.
// - Naming convention: SCREAMING_SNAKE_CASE.
// - Must be a constant expression known at compile time.
const MAX_USERS: u32 = 100_000;  // `_` is a digit separator for readability
const PI: f64 = 3.14159;

fn main() {
    println!("=== Lesson 002: Mutability, Constants, Shadowing ===\n");

    // ----- 1. Immutable by default -----
    let x = 5;
    println!("x = {x}");
    // x = 6;  // ❌ error[E0384]: cannot assign twice to immutable variable

    // ----- 2. `mut` — opt in to mutation -----
    let mut score = 0;
    score += 10;
    score += 25;
    println!("score after two increments: {score}");

    // ----- 3. Constants -----
    println!("MAX_USERS = {MAX_USERS}, PI = {PI}");

    // Differences vs `let`:
    //   let:   may be mutable (with `mut`), type optional, snake_case,
    //          can hold a runtime value, scoped to a block.
    //   const: never mutable, type REQUIRED, SCREAMING_SNAKE_CASE,
    //          must be compile-time constant, can be global.

    // ----- 4. Shadowing — reuse a name with `let` -----
    let n = 5;
    let n = n + 1;       // brand-new variable, shadows the previous
    let n = n * 2;
    println!("after shadowing: n = {n}");   // 12

    // ----- 5. Shadowing can CHANGE TYPE — `mut` cannot -----
    let spaces = "   ";          // &str
    let spaces = spaces.len();   // now usize — type changed via shadow ✅
    println!("number of spaces: {spaces}");

    // The following would fail because `mut` requires the same type:
    //     let mut spaces = "   ";
    //     spaces = spaces.len();  // ❌ expected &str, found usize

    // ----- 6. Block scope and shadowing -----
    let y = 10;
    {
        let y = y * 100;         // shadow only inside this block
        println!("inside block: y = {y}");   // 1000
    }
    println!("outside block: y = {y}");      // 10 — original unchanged
}
