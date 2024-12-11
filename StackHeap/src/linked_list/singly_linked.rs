use std::fmt::Display;

pub struct SinglyLinked<T> {
    pub elem: T,
    pub next: Option<Box<SinglyLinked<T>>>,
}

impl<T: Display> SinglyLinked<T> {
    pub fn new(item: T) -> Self {
        SinglyLinked {
            elem: item,
            next: None,
        }
    }
    pub fn print(self) {
        print!("{} ", self.elem);
        while let Some(ref item) = self.next {
            print!("{} ", (*item).elem);
        }
    }
    pub fn insert_from_head(&mut self, item: T) {
        let new_node = Box::new(SinglyLinked {
            elem: item,
            next: self.next.take(),
        });
        self.next = Some(new_node);
    }
}
