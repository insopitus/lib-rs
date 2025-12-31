use std::rc::{Rc, Weak};

pub struct DoublyLinkedListNode {
    _prev: Option<Rc<DoublyLinkedListNode>>,
    _next: Option<Weak<DoublyLinkedListNode>>,
}

pub struct DoublyLinkedList {
    _head: Option<DoublyLinkedListNode>,
    _tail: Option<DoublyLinkedListNode>,
}
impl DoublyLinkedList {}
