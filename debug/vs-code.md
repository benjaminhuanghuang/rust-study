https://dev.to/rogertorres/debugging-rust-with-vs-code-11dj


## MacOS X
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in library 'yourprogram'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--lib",
                    "--package=yourprogram"
                ],
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

## Windows:
```
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "(Windows) Launch",
            "type": "cppvsdbg",
            "request": "launch",
            "program": "${workspaceRoot}/target/debug/yourprogram.exe",
            "args": [],
            "stopAtEntry": false,
            "cwd": "${workspaceFolder}",
            "environment": [],
            "console": "externalTerminal"
        }
    ]
}
```