# Rust Module System

- Modules
- Path
- Crate
- Package

## Modules

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
