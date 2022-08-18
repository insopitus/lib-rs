pub struct DoublyLinkedList<T> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn append(&mut self, elem: T){
        todo!()
    }
    pub fn prepend(&mut self, elem:T){
        todo!()
    }
}

struct Node<T> {
    elem: T,
    next: Box<Option<Node<T>>>,
    prev: Box<Option<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(item: T)->Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
