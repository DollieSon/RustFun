use std::fmt::Display;

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

    pub fn print_all(self) {
        let mut curr = self.head;
        while let Some(item) = curr {
            print!("{} ", item.item);
            curr = item.next;
        }
    }

    pub fn insert_from_head(&mut self, item: T) {
        let new_node = Box::new(Node {
            item: item,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
}
