Rust's compile-time checking is done to protect against the following memory errors:

- Use after frees: This is where memory is accessed once it has been freed, which can
cause crashes. It can also allow hackers to execute code via this memory address.
- Dangling pointers: This is where a reference points to a memory address that no
longer houses the data that the pointer was referencing. Essentially, this pointer now
points to null or random data.
- Double frees: This is where allocated memory is freed, and then freed again.
This can cause the program to crash and increases the risk of sensitive data being
revealed. This also enables a hacker to execute arbitrary code.
- Segmentation faults: This is where the program tries to access the memory it's not
allowed to access.
- Buffer overrun: An example of this is reading off the end of an array. This can cause
the program to crash.
