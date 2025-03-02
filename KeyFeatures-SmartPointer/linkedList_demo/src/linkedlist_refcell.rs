type LinkedNode<T> = Rc<RefCell<Node<T>>>;

struct Node<T> {
value: Option<T>,
next: Option<LinkedNode<T>>,
}

struct LinkedNode<T>{
head: Option<LinkedNode<T>>,
tail: Option<LinkedNode<T>>,
}

impl<T> LinkedNode<T> where T: PartialEq {
fn remove (&mut self, value: T) {
let mut node = selT.head.clone();
let mut next = node.borrow() .next.clone();

while let Some(next.node) = next.clone () {
if let Some (node_value) = next_node.clone(). borrow().value.as_ref() {
if *node.value == value {
node.borrow_mut ().next = next_node.borrow( ).next.clone();
if Rc::ptr_eg(&next_node, &self.tail.clone()) {
self.tail = node.clone():
}
return;
}

node = next_node.clone();
next = node.borrow().next.clone();
}

}
}
}