# Rust Programming Exercises: Double Linked List

https://www.youtube.com/watch?v=k0cL6K28SL0


## Rc<T>, the Reference Counted Smart Pointer

The type Rc<T> provides shared ownership of a value of type T, allocated in the heap.

Invoking clone() on Rc produces a new pointer to the same allocation in the heap. 

When the last Rc pointer to a given allocation is destroyed, the value stored in that allocation is also dropped.

By default, the shared references is immutable. If you need mutability, put a `Cell` or `RefCell` inside the Rc

Rc cannot be sent between threads, and consequently Rc does not implement Send. If you need multi-threaded, atomic reference counting, use sync::Arc

A cycle between Rc pointers will never be deallocated. For this reason, Weak is used to break cycles. For example, a tree could have strong Rc pointers from parent nodes to children, and Weak pointers from children back to their parents.