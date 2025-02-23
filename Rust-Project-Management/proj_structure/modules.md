Files and folders are handled as modules in Rust.

If you want to declare that a module is in the another_file.rs file, you'll need to add it to your file:

```
  mod another_file;
```

you can only declare modules whose files are on the same level as your current module or file

Declare modules in sub folder:

```
|- src/
    |
    |- main.rs
    |- subfolder/
        |- another_file.rs
```

Add a mod.rs file into the subfolder folder. Declare another_file into mod.rs.The mod.rs files are mainly used for re-exporting modules' content outside.

```
  # mod.rs
  pub mod another_file;
```

Declare subfolder into main.rs.

```
  mod subfolder;

  use subfolder::another_file::some_function;
```
