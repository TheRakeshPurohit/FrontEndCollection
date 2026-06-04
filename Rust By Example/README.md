# 📚 Rust By Example — Yueli's executable learning notes

A Rust learning project where **each lesson lives in a single file** that is both
runnable code AND heavily-commented study notes.

## How to use

```bash
# Run any specific lesson
cargo run --bin 001-variables
cargo run --bin 004-functions

# Run the scratchpad (src/main.rs — current exercise)
cargo run

# Quickly type-check everything (no run)
cargo check --all-targets

# Lint with clippy
cargo clippy --all-targets
```

## Project layout

```
rust_by_example/
├── Cargo.toml
├── README.md            ← you are here
└── src/
    ├── main.rs          ← scratchpad for the lesson I'm currently working on
    └── bin/             ← polished, annotated lesson notes (one file per lesson)
        ├── 001-variables.rs
        ├── 002-mutability.rs
        ├── 003-data-types.rs
        ├── 004-functions.rs
        └── ...
```

## Curriculum (following The Rust Book)

### ✅ Phase 1 — Foundations (Book Ch. 3)
| # | Lesson | Topic |
|---|--------|-------|
| 001 | [`001-variables.rs`](src/bin/001-variables.rs) | `let` bindings, type inference, `println!` macros |
| 002 | [`002-mutability.rs`](src/bin/002-mutability.rs) | `mut`, `const`, shadowing |
| 003 | [`003-data-types.rs`](src/bin/003-data-types.rs) | Scalar & compound types |
| 004 | [`004-functions.rs`](src/bin/004-functions.rs) | Functions, statements vs expressions |
| 005 | _coming up_ | Control flow (if, loop, while, for) |

### 🚧 Phase 2 — Ownership (Book Ch. 4)
- 006 — Ownership & move semantics
- 007 — References & borrowing
- 008 — Slices

### 🚧 Phase 3+ — Structs, Enums, Modules, Collections, Errors, Traits, …
Will be added as I work through each lesson.

## Conventions

- Each lesson file starts with `//!` crate-level doc comments explaining the topic.
- Section dividers use `// ----- N. Title -----`.
- Each file has a working `fn main()` so it runs standalone.
- File names use `NNN-topic.rs` (zero-padded so they sort naturally).

## Resources
- 📖 [The Rust Book](https://doc.rust-lang.org/book/) — `rustup doc --book` for offline
- 🔬 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- 🦀 [Rustlings exercises](https://github.com/rust-lang/rustlings)
- 📚 [Standard library docs](https://doc.rust-lang.org/std/) — `rustup doc --std`
