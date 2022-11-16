String literals (e.g., "Rust in Action") have the type &str. 
The full type signature including the lifetime parameter is &'static str. 
The 'static lifetime is somewhat special. It too owes its name to implementation details. 
Executable programs can contain a section of memory that is hard-coded with values. That section is known as static memory because it is read-only during execution