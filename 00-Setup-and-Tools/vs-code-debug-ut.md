# Debugging Rust UT with VS Code

## Add project to workspace

```toml
[workspace]
members = [
  "Projects/rust-interpreter"
]
```

Then you can set "Run Test | Debug" in on the top of the test function

## Setup launch.json

https://dev.to/rogertorres/debugging-rust-with-vs-code-11dj

```json
// File copied from MacOS X
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug unit tests in library 'yourprogram'",
      "cargo": {
        "args": ["test", "--no-run", "--lib", "--package=yourprogram"],
        "filter": {
          "name": "yourprogram",
          "kind": "lib"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```
