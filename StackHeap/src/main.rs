pub mod linked_list;

fn main() {
    test5();
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

fn test4() {
    use linked_list::singly_linked::*;
    let mut list = SinglyLinked::new(21);
    list.insert_from_tail(23);
    list.insert_from_tail(31);
    list.insert_from_tail(41);
    list.print_all();
    list.print_all();
    println!("{} {} {}", list.pop(), list.pop(), list.pop());
    list.print_all();
}
//reverse LL test
fn test5() {
    use linked_list::singly_linked::*;
    let mut list = SinglyLinked::new(12);
    list.insert_from_tail(14);
    list.insert_from_tail(21);
    list.insert_from_tail(30);
    list.print_all();
    list.reverse_iterator();
    list.print_all();
}
