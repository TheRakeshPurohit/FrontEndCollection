//! # Lesson 003: Scalar & Compound Data Types
//!
//! Topics:
//! - Integers: i8..i128, u8..u128, isize/usize (default `i32`)
//! - Floats: f32, f64 (default `f64`)
//! - Booleans, characters (Unicode scalar values, 4 bytes!)
//! - Tuples and arrays (compound types)
//! - Arithmetic, integer overflow handling, debug vs display printing
//!
//! Run with: `cargo run --bin 003-data-types`

fn main() {
    println!("=== Lesson 003: Scalar & Compound Data Types ===\n");

    // ----- 1. Integers -----
    // Signed:   i8 i16 i32 i64 i128 isize    (can be negative)
    // Unsigned: u8 u16 u32 u64 u128 usize    (>= 0 only)
    // Default when ambiguous: i32. `usize` is used for indexing collections.
    let signed: i32 = -42;
    let unsigned: u8 = 255;
    let big: i64 = 9_000_000_000;
    let hex = 0xff_u32;            // hex literal
    let octal = 0o77_u32;          // octal literal
    let binary = 0b1010_0001_u8;   // binary literal
    let byte = b'A';               // byte literal (u8, value 65)
    println!("ints: {signed} {unsigned} {big} {hex} {octal} {binary} {byte}");

    // ----- 2. Integer overflow -----
    // Debug builds: PANIC on overflow.
    // Release builds: wrap silently (two's-complement).
    // For explicit control, use these methods:
    let n: u8 = 250;
    println!("wrapping_add:    {} (250 + 10 wraps to 4)", n.wrapping_add(10));
    println!("checked_add:     {:?} (returns None on overflow)", n.checked_add(10));
    println!("saturating_add:  {} (clamps at u8::MAX = 255)", n.saturating_add(10));
    let (val, overflowed) = n.overflowing_add(10);
    println!("overflowing_add: ({val}, {overflowed})");

    // ----- 3. Floats -----
    let pi: f64 = 3.14159;           // f64 is default
    let e: f32 = 2.71828;            // explicit f32
    let scientific = 1e6;            // 1,000,000.0
    println!("floats: {pi} {e} {scientific}");

    // ----- 4. Booleans & characters -----
    let t: bool = true;
    let f: bool = false;
    let letter: char = 'R';          // single quotes for char, NOT "R"
    let crab: char = '🦀';           // 4 bytes! char is a Unicode scalar value
    let newline: char = '\n';
    println!("bools: {t} {f}  | chars: '{letter}' '{crab}' (size of char in bytes = {})",
             std::mem::size_of::<char>());
    let _ = newline;  // (just to acknowledge it; we don't use it here)

    // ----- 5. Tuples — fixed-size, MIXED types -----
    let person: (&str, u32, f64) = ("Alice", 30, 1.65);
    println!("person.0 = {}, person.1 = {}, person.2 = {}",
             person.0, person.1, person.2);

    // Destructuring
    let (name, age, height) = person;
    println!("destructured: {name}, age {age}, {height}m");

    // The unit type `()` is an empty tuple — means "no value".
    let _nothing: () = ();

    // ----- 6. Arrays — fixed-size, SAME type, on the STACK -----
    let primes: [i32; 5] = [2, 3, 5, 7, 11];   // type is [i32; 5]
    let zeros = [0; 10];                       // [0; N] means "N copies of 0"
    println!("primes = {:?}, len = {}", primes, primes.len());
    println!("zeros  = {:?}", zeros);
    println!("first prime = {}, last prime = {}", primes[0], primes[primes.len() - 1]);

    // Out-of-bounds access PANICS at runtime — memory-safe, no buffer overflow.
    // println!("{}", primes[99]);  // ❌ would panic

    // ----- 7. Arithmetic -----
    let a: i32 = 10;
    let b: i32 = 3;
    println!("{a} + {b} = {}", a + b);
    println!("{a} - {b} = {}", a - b);
    println!("{a} * {b} = {}", a * b);
    println!("{a} / {b} = {}  (integer division TRUNCATES toward zero)", a / b);
    println!("{a} % {b} = {}", a % b);

    let fa: f64 = 10.0;
    let fb: f64 = 3.0;
    println!("{fa} / {fb} = {}  (float division is real division)", fa / fb);

    // ----- 8. Sum a slice (preview of iterators — Lesson 025) -----
    let sum: i32 = primes.iter().sum();
    println!("sum of primes = {sum}");
}
