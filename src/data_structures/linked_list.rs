use std::rc::{Rc, Weak};

pub struct DoublyLinkedListNode{
    prev:Option<Rc<DoublyLinkedListNode>>,
    next:Option<Weak<DoublyLinkedListNode>>,
}

pub struct DoublyLinkedList{
    head:Option<DoublyLinkedListNode>,
    tail:Option<DoublyLinkedListNode>
}
impl DoublyLinkedList{
    
}