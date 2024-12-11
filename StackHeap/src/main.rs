pub mod linked_list;

fn main() {
    println!("Hello World!");
}

fn test1() {
    use linked_list::singly_linked::*;
    let list: SinglyLinked<i32> = SinglyLinked {
        elem: 2,
        next: None,
    };
}
