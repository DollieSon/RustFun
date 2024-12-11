pub struct SinglyLinked<T> {
    pub elem: T,
    pub next: Option<Box<SinglyLinked<T>>>,
}
