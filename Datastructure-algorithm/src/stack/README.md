# Stack in Rust

Rust’s `Vec<T>` already supports efficient push (for push_back) and pop (for pop_back).
A separate `Stack<T>` would be redundant.

## Different Implementations Have Different Trade-Offs

`Vec<T>` Faster due to contiguous memory.
`LinkedList<T>` Slower but avoids reallocations.
