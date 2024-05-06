use std::collections::VecDeque;
use crate::mytraits::collections::{Queue, Deque};

pub struct MyDeque {
    data: VecDeque<i32>
}

impl MyDeque {

    pub fn new() -> MyDeque {
        let data = VecDeque::<i32>::new();
        MyDeque {data}
    }
}

impl Queue for MyDeque {
    
    fn len(&self) -> usize {
        self.data.len()
    }

    fn push_back(&mut self, n: i32) {
        self.data.push_back(n)
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.data.pop_front()
    }
}

impl Deque for MyDeque {
    
    fn push_front(&mut self, n: i32) {
        self.data.push_front(n)
    }

    fn pop_back(&mut self) -> Option<i32> {
        self.data.pop_back()
    }
}