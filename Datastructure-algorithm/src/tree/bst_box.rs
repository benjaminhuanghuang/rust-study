use std::cmp::PartialOrd;
use std::cmp::PartialEq;

struct Tree<T>
    where T: PartialEq + PartialOrd
{
    v: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T> Tree<T>
    where T: PartialEq + PartialOrd
{
    pub fn insert(&mut self, x: T) {
        if x == self.v {
            return;
        }

        let mut next_node = if x < self.v {
            &mut self.left
        } else {
            &mut self.right
        };

        match next_node {
            &mut Some(ref mut node) => node.insert(x),
            &mut None => {
                *next_node = Some(Box::new(Tree {
                    v: x,
                    left: None,
                    right: None,
                }));
            }
        }
    }

    pub fn is_elem(&self, x: T) -> bool
        where T: PartialEq + PartialOrd
    {
        if x == self.v {
            return true;
        }

        let next_node = if x < self.v {
            &self.left
        } else {
            &self.right
        };

        match next_node {
            &Some(ref node) => node.is_elem(x),
            &None => false,
        }
    }

    pub fn new(x: T) -> Tree<T> {
        Tree {
            v: x,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let mut tree = Tree::<i32>::new(9);
    
    tree.insert(7);
    tree.insert(99);
    tree.insert(300);
    tree.insert(3);
    
    println!("5 `is_elem` Tree {}", tree.is_elem(5));
    println!("300 `is_elem` Tree {}", tree.is_elem(300));
}