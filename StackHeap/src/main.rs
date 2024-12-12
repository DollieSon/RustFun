pub mod linked_list;

fn main() {
    test3();
    // println!("Hello World!");
}

fn test1() {
    use linked_list::singly_linked::*;
    // let list: SinglyLinked<i32> = SinglyLinked {
    //     elem: 2,
    //     next: None,
    // };
}

fn test2() {
    use linked_list::singly_linked::*;
    let mut list = SinglyLinked::new(21);
    list.insert_from_head(13);
    list.insert_from_head(31);
    list.print_all();
}

fn test3() {
    use linked_list::singly_linked::*;
    let mut list = SinglyLinked::new(21);
    list.insert_from_tail(23);
    list.insert_from_tail(31);
    list.insert_from_tail(41);
    list.print_all();
}
