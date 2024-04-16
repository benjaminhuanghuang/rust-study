use std::collections::HashMap;
use std::hash::Hash;
use std::{cell::RefCell, rc::Weak};

mod double_linked_list;
use double_linked_list::{DoubleLinkedList, DoubleLinkedListNode};

struct LRUCache<K: Copy + Eq + Hash, T: Copy> {
  dlist: DoubleLinkedList<T>,
  map: HashMap<K, Weak<RefCell<DoubleLinkedListNode<T>>>>,
  capacity: usize,
}

impl<K: Copy + Eq + Hash, T: Copy> LRUCache<K, T> {
  pub fn new() -> Self {
    LRUCache::with_capacity(10)
  }

  pub fn with_capacity(capacity: usize) -> Self {
    LRUCache {
      dlist: DoubleLinkedList::new(),
      map: HashMap::new(),
      capacity,
    }
  }

  pub fn get(&mut self, key: K) -> Option<T> {
    let ptr = self.map.get_mut(&key);
    if ptr.is_none() {
      return None;
    }

    let ptr = ptr.unwrap();
    let ptr = ptr.upgrade();
    match ptr {
      None => None,
      Some(node) => {
        let value = node.borrow().value;
        self.dlist.move_node_to_back(node);

        Some(value)
      }
    }
  }

  pub fn put(&mut self, key: K, value: T) {
    let ptr = self.map.get_mut(&key);

    let ptr = if ptr.is_some() {
      ptr.unwrap().upgrade()
    } else {
      None
    };

    match ptr {
      None => {
        self.dlist.push_back(value);
        if let Some(tail) = self.dlist.get_weak_tail() {
          self.map.insert(key, tail);
        }

        if self.dlist.len() > self.capacity {
          self.dlist.pop_front();
        }
      }
      Some(node) => {
        node.borrow_mut().value = value;
        self.dlist.move_node_to_back(node);
      }
    }
  }
}
/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn works_lru() {
    let mut lru_cache = LRUCache::new();
    lru_cache.put(1, "foo");
    lru_cache.put(2, "bar");
    lru_cache.put(3, "fizz");
    lru_cache.put(4, "buzz");
    lru_cache.put(5, "bazz");

    assert_eq!(lru_cache.get(3), Some("fizz"));
    assert_eq!(lru_cache.get(2), Some("bar"));
    assert_eq!(lru_cache.get(5), Some("bazz"));
  }

  #[test]
  fn test_lru_capacity() {
    let mut lru_cache = LRUCache::with_capacity(3);
    lru_cache.put(1, "foo");
    lru_cache.put(2, "bar");
    lru_cache.put(3, "fizz");
    lru_cache.put(4, "buzz");
    lru_cache.put(5, "bazz");

    assert_eq!(lru_cache.get(3), Some("fizz"));
    assert_eq!(lru_cache.get(4), Some("buzz"));
    assert_eq!(lru_cache.get(1), None);
  }
}
