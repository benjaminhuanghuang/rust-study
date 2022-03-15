Smart Pointer:
- on statck
- contains meta data and a point to heap
- works as a reference
- drop



## Box
```
type TestResult = Result<(), Box<dyn std::error::Error>>;
```
Box indicates that the error will live inside a pointer where the memory is dynamically
allocated on the heap rather than the stack, 

dyn indicates that the method calls on the std::error::Error trait are dynamically dispatched.

## Rc