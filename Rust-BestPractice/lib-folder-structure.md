# Folder structure of lib

In a Rust project, a folder (module) can contain multiple submodules, and the entry file for the module can follow one of these conventions:

Case 1: Using mod.rs (Older Style)
Before Rust 2018, you had to place the module's entry file inside the folder as mod.rs. Example:

```rust
src/
 ├── main.rs
 ├── my_module/
 │    ├── mod.rs   <-- Acts as `my_module`'s main file
 │    ├── sub_module.rs
```

In main.rs:

```rust
mod my_module; // Looks for `my_module/mod.rs`
```

in Rust, when a file has the same name as a folder, that file typically serves as the module entry point (like a "lib" file) for the folder.

Case 2: Using the Folder Name as a File (Rust 2018+)

```rust
src/
 ├── main.rs
 ├── my_module.rs   <-- Acts as `my_module`'s main file
 ├── my_module/     <-- Folder for submodules
 │    ├── sub_module.rs
```

In main.rs:

```rust
mod my_module;  // Looks for `my_module.rs`
```
