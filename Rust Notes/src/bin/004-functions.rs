//! # Lesson 004: Functions
//!
//! Topics:
//! - `fn` syntax, parameter types (required), snake_case convention
//! - Return values via `->` and implicit (no-semicolon) last expression
//! - Early `return` for short-circuits
//! - **Statements vs expressions** — the big idea
//! - The unit type `()`
//! - Returning multiple values via tuples
//! - Block expressions
//!
//! Run with: `cargo run --bin 004-functions`

fn main() {
    println!("=== Lesson 004: Functions ===\n");

    // ----- 1. Basic call -----
    greet("World");

    // ----- 2. Functions with return values -----
    let s = add(3, 4);
    println!("3 + 4 = {s}");

    // ----- 3. Early return -----
    println!("abs(-7) = {}", abs(-7));
    println!("abs( 7) = {}", abs(7));

    // ----- 4. Statements vs expressions -----
    // A STATEMENT does something but returns no value (`let x = 5;`).
    // An EXPRESSION evaluates to a value (`5`, `1 + 2`, `{ ... }`, `if c { a } else { b }`).
    // Adding a `;` turns an expression into a statement, discarding the value.
    //
    // A block `{ ... }` IS an expression. Its value is its last expression.
    let block_value = {
        let a = 10;
        let b = 20;
        a + b              // no semicolon → block evaluates to 30
    };                     // the semicolon here ends the `let` statement
    println!("block_value = {block_value}");

    // Common mistake: writing `n * n;` (with semicolon) in a function body that
    // expects to return an i32 → compiler error "expected i32, found ()".
    // The rule: the LAST expression's value (no semicolon) becomes the return value.

    // ----- 5. Returning multiple values via tuples -----
    let (lo, hi) = min_max(&[5, 2, 9, 1, 7]);
    println!("min = {lo}, max = {hi}");

    // ----- 6. Convention: implicit return vs explicit `return` -----
    //   Final value  → no `return`, no semicolon (preferred)
    //   Early exit   → `return value;`
    // Both compile to identical code. Clippy will warn about `needless_return`
    // when you use `return` for the final value.
    println!("square_then_add_one(5) = {}", square_then_add_one(5));
}

// ----- Function definitions -----
// Order doesn't matter — Rust resolves names within a module.

/// Greets the given name. Returns unit `()` implicitly (no `->`).
fn greet(name: &str) {
    println!("Hello, {name}!");
}

/// Adds two integers. The last expression (no semicolon) is the return value.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Absolute value. Demonstrates an EARLY return.
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;    // early return needs `return` and `;`
    }
    x                 // implicit final return
}

/// Returns (min, max) from a slice of integers.
/// `&[i32]` is an array slice — a borrowed view (covered in Lesson 008).
fn min_max(nums: &[i32]) -> (i32, i32) {
    let mut min = nums[0];
    let mut max = nums[0];
    for &n in nums {  // `&n` destructures &i32 into i32 — see Lesson 007
        if n < min { min = n; }
        if n > max { max = n; }
    }
    (min, max)
}

/// Demonstrates a BLOCK EXPRESSION used inside a function body.
fn square_then_add_one(n: i32) -> i32 {
    let squared = {
        // any logic here; the block evaluates to its last expression
        n * n
    };
    squared + 1   // implicit return
}
