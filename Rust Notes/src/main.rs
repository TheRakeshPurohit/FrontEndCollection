//! # Rust Notes — Yueli's executable learning reference
//!
//! Each file under `src/bin/` is a self-contained lesson:
//! it is BOTH runnable demo code AND your study notes.
//!
//! To run a specific lesson:
//!     cargo run --bin 001-variables
//!     cargo run --bin 004-functions
//!
//! To list all available lessons:
//!     cargo run                  (you're seeing it now)

fn main() {
    println!("📚 Rust Notes — your executable learning reference\n");
    println!("Run any lesson with:  cargo run --bin <lesson-name>\n");
    println!("Lessons available:");
    println!("  001-variables       — let bindings, type inference, println!");
    println!("  002-mutability      — mut, const, shadowing");
    println!("  003-data-types      — scalars (int/float/bool/char), tuples, arrays");
    println!("  004-functions       — fn syntax, returns, statements vs expressions");
    println!("\nMore lessons will appear here as you progress. See README.md for full TOC.");
}
