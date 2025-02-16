# Clap

## Add dependencies

```sh
cargo add clap@2
```

Cargo.toml

```toml
[dependencies]
clap = "2"
```

## Run

```sh
cargo run -- picture

cargo run -- --help
```

## Optional argument

the -n is an optional argument because you can leave it out

program options common to have short names with one dash and a single
character, like -h for the help flag, and long names with
two dashes and a word, like --help

## positional arguments

positional arguments's position relative to the name of the program (the first
element in the arguments)

## flag argument

The options -n and -h are often called `flags` because
they don’t take a value. Flags have one meaning when present
and the opposite when absent
