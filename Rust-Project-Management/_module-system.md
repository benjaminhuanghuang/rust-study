# Rust Module System

- Modules
- Path
- Crate
- Package

## Modules

In Rust, all files and folders are modules.

In order to use the code in a module, you need to first import it with the `mod` syntax
When you import a module with the mod statement, Rust automatically creates a module namespace for it (to avoid conflicts) and thus we cannot access the struct type or function in that file directly.
The `use` keyword is a convenience to map a fully qualified type name to just it’s type name
crate:: means the root of the module tree

```rs
mod my_struct
use crate::my_struct::MyStruct;
```

- Subdivide code into group related items
- Provide isolated `namespace` to control `scope` and `privacy`, For example, put functions in module file_io and network_io

Modules can contain:

- Functions
- Macros
- Types
- Traits
- impl blocks
- Constants
- Modules ...

## Module visibility

Everything inside a module (ie, a file or subfolder within the /src folder) can access anything else within that module.(same module)
private functions within a module are still accessible for tests within that module (idiomatic Rust keeps unit tests within the module).

Everything outside a module can only access public members of that module.

## Path

- Reference items within the module system
- can be relative or absolute

## Crate

Type of crates:

- Binary crates
- Library crates

- Binary crates
  Binary crates compile to produce an executable program

- Library crates
  Library crates contain code for other programs to use

The code in libs can be accesse in whole crate

```
  <catr>::<funcation>()
```

By default, all the variables and functions in a module are private, which means they are accessible only to other code within the same module.

## Package

Used to build, test and share crates with Cargo.

Contains one or more target crate.

- Up to one library crate
- Zero or more binary crates

Consists of:

- Cargo.toml
- src directory
- target directory

- Contains many crates and one library crate.
- The root directory of a project is the package name by default.
- each file within the /src/bin folder represent additional binary crates.

Create a package:

```sh
cargo new <project-name>
```

## Package and Crate

- Package is a collection of crates
