use std::thread;
#[derive(Debug)]
enum LinkedList<T> {
    Cons(T, Box<LinkedList<T>>), //"Cons" is nothing its just a name we call the pair
    Nul,                         // Nul also does not have a meaning
}
impl<T> LinkedList<T> {
    fn spawn(item: T) -> LinkedList<T> {
        LinkedList::Cons(item, Box::new(LinkedList::Nul))
    }
    fn add_elem(&mut self, item: T) {
        let &mut head = self;
        loop {
            match self {
                LinkedList::Cons(item, Next) => {
                    println!("{item}")
                }
                _ => {
                    break;
                }
            }
        }
    }
}
fn main() {
    let something = LinkedList::spawn(12);
    println!("{:?}", something)
}
