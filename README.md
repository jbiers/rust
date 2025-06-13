# Notes from The Rust Book

## Useful cargo commands
- `cargo new <dirname>`
- `cargo build`
- `cargo build --release`
- `cargo run`
- `cargo check`
- `cargo update`
- `cargo doc --open`

Rust [prelude](https://doc.rust-lang.org/std/prelude/index.html) - stuff from the std library which are included by default in every program.

Variables in Rust are immutable by default. The same is true for references. When passing a reference with `&` you need to explicitly make it mutable if that is needed.

*Binary crates* are executable packages, while *library crates* provide code to be used by other programs.

Shadowing variable names :thinking_face:
