pub struct BinaryTree<T> {
    _root: Node<T>,
}

pub struct Node<T> {
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>,
    pub value: T,
}
impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            left_child: None,
            right_child: None,
            value,
        }
    }
    pub fn set_left(&mut self, v: T) {
        self.left_child = Some(Box::new(Self::new(v)));
    }
    pub fn set_right(&mut self, v: T) {
        self.right_child = Some(Box::new(Self::new(v)));
    }
    pub fn clear_left(&mut self) {
        self.left_child = None;
    }
    pub fn clear_right(&mut self) {
        self.right_child = None
    }
}
