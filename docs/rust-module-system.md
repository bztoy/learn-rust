# Rust Module System

- Package: A bundle of 1+ craits
- Crate: Tree of modules. It can be a library or executable.
- Module/use: for paths (organization, scope, privacy)
- Paths: to name items (struct, function, module)

### Package

Cargo's feature to build, test, and share Crates.
It contains a Cargo.toml file

### Crate

The smallest amount of code that the Rust compiler considers at a time.

Crate root
A crate root is a source file to create root modules.

src/main.rs is the crate root of a binary crate with the same name as the package.

### Module/use

Modules are defined to control **_scope_** and **_privacy_**

A filename can be both a module or submodule.
A directory is a module but it needs

- A Rust file named <dir_name>.rs located in the same level in the path to declare those submodules inside this module.
- A mod.rs file inside the folder.

7 rules for Modules

1. Start from the crate root (src/main.rs)
2. How to declare a module

- inline
- src/<mod>/<submod.rs>
- src/<mod>/mod.rs

3. declaring submodules
4. Paths to code in modules
5. The use keyword

The use keyword creates shortcuts to items to reduce repetition of long path.

instead of typing every time
crate::<mod>::<submod>::<items>

create a shortcut
use crate::<mod>::<submod>::<items>

6. Private vs Public

- By default, the code within a module is private from its parent modules.
- To make a module public, declare it with pub mod instead of mod.
- To make items within a public module public as well, use pub before their declarations.

7. Grouping in Modules

### Paths

relative path can be used with the super keyword to access parent moudle.

Path for structure
A structure can have a mix of private and public fields.

Path for enum
can set the visibility at the emum only, cannot set at its member.

### Use keyword

wildcard shortcut creation
use crate::..::..::\*;

how to use modules with the same name with as keyword
use std::fmt::Result;
use std::io::Result as IoResult;

Re-export the name
pub use <module>

### How to use external packages

nested paths
use std::io;
use std::cmp::Ordering;

use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
use std::io::{self, Write};

The Global operator \*
use std::collections::\*;
