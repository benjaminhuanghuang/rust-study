# difference between using Rc<RefCell<TreeNode>> and Option<Box<ListNode>>

Linked lists typically don't need shared ownership of nodes.
Each node is owned by exactly one part of the structure (the previous node owns the next node,
and the next node doesn't need to be accessed by multiple owners).

Thus, Box is enough to manage ownership of the next node, and Rust automatically handles the cleanup.

In trees, however, multiple nodes (like left and right children) might need to share ownership of the same tree node.
To handle this, we use Rc for shared ownership and RefCell to allow mutation.

Rc allows shared ownership of nodes, and RefCell allows those nodes to be mutated even when shared, while maintaining Rust’s safety guarantees.

A tree node might be accessed by both its parent (to update the child pointers) and its children (to modify the node's value).
Rc allows shared ownership of nodes, and RefCell allows those nodes to be mutated even when shared, while maintaining Rust’s safety guarantees.
