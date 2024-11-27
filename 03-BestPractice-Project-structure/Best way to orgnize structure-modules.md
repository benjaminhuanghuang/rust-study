https://users.rust-lang.org/t/best-way-to-organize-structure-modules-in-project/114883/3

Modules are the basis of encapsulation and privacy in Rust: Things that are private are always private to a particular module (rather than private to a particular type like in many OO languages).
You should use a module anywhere you want to create some interface that hides its internals.

Where you might write a class with private members/methods in Java- or C++-like languages, in Rust you should frequently write a module instead.

Modules also form the namespace of your Rust crate, letting you group similar things together (much like C++ or Java namespaces). This lets you organize the documentation of your library crates, for example. But this is a bit of a secondary concern, since you can always use re-exports to make your public namespace structure different from your “real” code structure if desired.
