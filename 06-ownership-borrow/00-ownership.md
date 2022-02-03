
Rust stores data in two differently strudture parts of memory


## Stack vs Heap
Stack
- Last In, first Out
- Memory is auto recaptured by the program after variables go out of scope
- Is default in Rust
- Fixed size variables. Colleciotn can NOT be stack based ( Strings are a collection of u8)


Heap
- Flexibility
- Memory that can grow in size (Vector, HeashMap, String, etc)
- Memory can live beyond the scope.
- Heap memory is auto recaptured whtn the last OWNER of the memory goes out the scope.

