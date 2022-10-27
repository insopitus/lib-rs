use std::{rc::{Weak, Rc}, cell::RefCell};

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
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
    pub fn append(&mut self, elem: T) {
        if let Some(tail) = &mut self.tail {
            todo!()
        } else {
            todo!()
        }
        if self.head.is_none() {
        } else {
        }
    }
    pub fn prepend(&mut self, elem: T) {
        todo!()
    }
    pub fn remove_first(&mut self) {
        todo!()
    }
    pub fn remove_last(&mut self) {}
}

struct Node<T> {
    pub elem: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(item: T) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = DoublyLinkedList::new();
        list.append(1);
        list.append(2);
        list.remove_first();
    }
}


