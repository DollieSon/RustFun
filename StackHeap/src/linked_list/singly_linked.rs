use std::{borrow::Borrow, fmt::Display};

pub struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

pub struct SinglyLinked<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T: Display> SinglyLinked<T> {
    pub fn new(elem: T) -> Self {
        let new_node = Some(Box::new(Node {
            item: elem,
            next: None,
        }));
        SinglyLinked {
            head: new_node,
            len: 1,
        }
    }

    pub fn print_all(&self) {
        let mut curr = &self.head;
        while let Some(item) = curr {
            print!("{} ", item.item);
            curr = &item.next;
        }
        print!("\n");
    }

    pub fn insert_from_head(&mut self, item: T) {
        let new_node = Box::new(Node {
            item: item,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }
    pub fn insert_from_tail(&mut self, item: T) {
        let new_node = Box::new(Node {
            item: item,
            next: None,
        });

        let mut curr = &mut self.head;
        while let Some(ref mut node) = curr {
            curr = &mut node.next;
        }
        *curr = Some(new_node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> T {
        match self.head.take() {
            None => panic!("List Is Empty"),
            Some(node) => {
                self.head = node.next;
                self.len -= 1;
                node.item
            }
        }
    }

    //reversing a LL through iteration
    // make current = head and prev = none
    // loop x
    // // temp = current.next
    // // current.next = prev
    // // prev = current

    pub fn reverse_iterator(&mut self) {
        let mut curr = self.head.take(); // we take
        let mut prev: Option<Box<Node<T>>> = None;
        while let Some(ref mut node) = curr {
            let next = node.next.take();
            match prev {
                None => {
                    node.next = None;
                }
                Some(_) => node.next = prev,
            }
            prev = curr;
            curr = next;
        }
        self.head = prev;
    }
    pub fn reverse_recursive(&mut self) {}
    //node.next = fn(node.next)
    fn reverse_recursive_helper(mut curr_node: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        if let Some(mut node) = curr_node {
            match node.next {
                Some(next) => {
                    node.next = Self::reverse_recursive_helper(node.next.take());
                }
                None => return curr_node,
            }
        }
        None
    }
    // pub fn pop_end(&mut self) -> T {
    //     let mut curr = self.head;
    // }
}
