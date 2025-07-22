# Introduction to Rust Programming language

rustup: the recommended way to manage Rust installation and update
trait: is a collection of Rust codes

Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module

## cargo

cargo

- cargo new // create a new directory
- cargo init // init inside a folder/directory
- cargo add <crait-name>
- cargo doc
- cargo test

## main keywords

fn - the fn keyword stands for a function, main function is required as an entry point.
use - use is using to import a library into a source file
mod - import code from other modules (usually files) into the source file

## comments

- Regular comments
  - // Line comments which go to the end of the lile
  - /\* Block comments which go to the closing delimiter \*/

// This is an example of a line comment.

- Doc comments (Markdown supported)
  - /// Generate library docs for the following item.
  - //! Generate library docs for the enclosing item.

## variables

create a variable with keyword let
a variable is imutable by default,if mutable is needed, using mut
naming convention: snake case

```rust
let hours_in_day = 24;
let mut sec = 5;
sec = 10;

```

const
type is required, global scope is fine
naming convention: All upper case

```rust
const ONE_MINUTE: i32 = 60;

```

variable shodowing is supported in Rust

```rust

let n = 20;
println!("n is {}", n);

{
    let n = 10;
    println!("n is {}", n);

}

// this also works
let spaces = "        "; //String
let spaces = spaces.len(); //usize

```

# Refference

[Rust by Example](https://doc.rust-lang.org/rust-by-example/)
