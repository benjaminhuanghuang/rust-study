# Generate documents

## Using rustdoc to render docs for a single source file

```sh
rustdoc ch3-file-doced.rs.
```

rustdoc creates a directory (doc/) for you.

The documentation’s entry point is actually within a subdirectory: doc/ch3_file_doced/index.html.

## Using cargo to render docs for a crate and its dependencies

```
cargo doc --open.
```
