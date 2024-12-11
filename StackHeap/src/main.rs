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

fn test2() {
    use linked_list::singly_linked::*;
    let list = SinglyLinked::new(21);
    list.print();
}
