- Values are owned by the variables assigned to them.
- As soon as the variable goes out of scope, it is deallocated from the memory it is
occupying.
- Values can be used by other variables, as long as we adhere to the following rules:
- Copy: This is where the value is copied. Once it has been copied, the new variable
owns the value, and the existing variable also owns its own value.
- Move: This is where the value is moved from one variable to another. However,
unlike clone, the original variable no longer owns the value.
- Immutable borrow: This is where another variable can reference the value of
another variable. If the variable that is borrowing the value falls out of scope, the
value is not deallocated from memory as the variable borrowing the value does not
have ownership.

- Mutable borrow: This is where another variable can reference and write the value
of another variable. If the variable that is borrowing the value falls out of scope, the
value is not deallocated from memory as the variable borrowing the value does not
have ownership.