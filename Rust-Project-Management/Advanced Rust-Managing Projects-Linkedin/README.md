# Advanced Rust: Managing Projects Manage and test Rust projects

By Barron Stone, 2022.3
https://www.linkedin.com/learning/advanced-rust-managing-projects

## Table of content

### Introduction

Manage and test Rust projects
What you should know

### Managing Projects

The module system

Packages vs. crates

Defining modules

Absolute vs. relative path

Public modules

Public structs and enums

Bringing paths into scope

Using external crates

Separating modules into multiple files

### Automating Tests

Test functions

assert! macro

Custom failure messages

assert_eq! and assert_ne! macros

should_panic! macro

Controlling test execution

Unit tests

Integration tests

Challenge: Write automated tests

Solution: Write automated tests

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

## Define modules

- inline

```rust
mod module_name {
    // items

    mod sub_module {
        // items
    }
}
```

- Separate file

- Privacy
  All items in a module (sub module and funcitons) are private by default

Parent modules `can not` use private items in a child module

Child modules `can` use item from parent modules

When made public all enum variants are all public.

To Structure, need to add pub to all fields.

## Path

- Reference items within the module system
- can be relative or absolute

```rust
// Absolute path
crate::hello::english::greet();

// Relative path
hello::english::greet();
supper::hello::english::greet();
```

Relatvie Path, starts from the current module. Can start from `self` or `super` or module name

尽量使用 relative path，以免 rename module 影响很多代码

- Bring path into scope: `use` statment

```
  use <a>::<b>
```
