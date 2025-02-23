# A Singly Linked List in Rust

https://www.youtube.com/watch?v=IiDHTIsmUi4

## Implementation 1

```rs
enum Node {
  Empty,
  NonEmpty(u32, Box<Node>),
}

let list = Node::NonEmpty(123, Box::new(Node::Empty));
```

There are 2 issues:

- We'are allocating a node that just says "I am not actually a Node"
- One of the nodes isn't heap allocated at all

## Implementation 1.1

```rs
struct Node {
  val: i32,
  next: List
}

enum List {
  Empty,
  NonEmpty(Box<Node>),
}


let list = List::NonEmpty(Box::new(Node {va: 123, next: List::Empty}));
```

## Implementation 1.1.2, use option

```rs
public struct LinkedList {
  head: Link
}

struct Node {
  val: i32,
  next: List
}

type Link = Option<Box<Node>>;

let list: LinkedList =  LinkedList {
  head: Some(Box::new(Node {va: 123, next: List::None}));
}
```

## Implementation 2.1

```rs
enum Node<'a> {
  Empty,
  NonEmpty(u32, &'a Node<'a>),
}

let node: &Node = &Node::Empty;
let list: Node = Node::NonEmpty(123, node);
```

## Implementation 2.2

```rs
enum Node {
  Empty,
  NonEmpty(u32, &'static Node),
}

let node: &Node = &Node::Empty;
let list: Node = Node::NonEmpty(123, node);
```
