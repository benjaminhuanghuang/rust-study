# A Singly Linked List in Rust (2020)
https://www.youtube.com/watch?v=IiDHTIsmUi4&t=30s&ab_channel=RyanLevick


https://doc.rust-lang.org/std/collections/struct.LinkedList.html
Struct std::collections::LinkedList

NOTE: It is almost always better to use Vec or VecDeque because array-based containers are generally faster, more memory efficient, and make better use of CPU cache.


- Learn Rust With Entirely Too Many Linked Lists
  - https://rust-unofficial.github.io/too-many-lists/

- https://www.bilibili.com/video/BV1eb4y1Q7FA/




## Implementation 1
```
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
```
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
```
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
```
enum Node<'a> {
  Empty,
  NonEmpty(u32, &'a Node<'a>),
}

let node: &Node = &Node::Empty;
let list: Node = Node::NonEmpty(123, node);
```



## Implementation 2.2
```
enum Node {
  Empty,
  NonEmpty(u32, &'static Node),
}

let node: &Node = &Node::Empty;
let list: Node = Node::NonEmpty(123, node);
```
