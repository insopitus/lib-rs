use std::collections::HashMap;

/// all huffman nodes for (value, weight) are leaves. the other nodes are for algorithm
#[derive(Debug, Default)]
pub struct Tree {
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
    /// the char/byte to be encoded
    value: Option<u8>,
    /// how many times the value appears
    weight: u32,
}

impl Tree {
    pub fn new(value: Option<u8>, weight: u32) -> Self {
        Self {
            left: None,
            right: None,
            value,
            weight,
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
    pub fn get_value(&self) -> Option<u8> {
        self.value
    }
    pub fn set_left_child(&mut self, tree: Tree) {
        self.weight += tree.weight;
        self.left = Some(Box::new(tree));
    }
    pub fn set_right_child(&mut self, tree: Tree) {
        self.weight += tree.weight;
        self.right = Some(Box::new(tree));
    }
}

pub fn encode(data: &[u8]) -> (Vec<u8>, Tree) {
    let mut freq_table = HashMap::<u8, u32>::new();
    // TODO: use cheaper hash function
    for byte in data {
        *freq_table.entry(*byte).or_insert(0) += 1;
    }
    let mut trees: Vec<Tree> = freq_table
        .into_iter()
        .map(|(value, weight)| Tree::new(Some(value), weight))
        .collect();
    assert!(&trees.len() != &0);
    // TODO if there is only one tree
    while trees.len() > 1 {
        let (left, right) = (
            take_lightest_tree(&mut trees).unwrap(),
            take_lightest_tree(&mut trees).unwrap(),
        );
        let mut parent_tree = Tree::new(None, 0);
        parent_tree.set_left_child(left);
        parent_tree.set_right_child(right);
        trees.push(parent_tree);
    }
    dbg!(&trees[0]);
    // combine the two trees

    // turn hash table into a array
    todo!()
}

pub fn decode(data: &[u8], tree_root: Tree) -> Vec<u8> {
    todo!()
}

/// remove the tree with smallest weight value from the list and return it.
fn take_lightest_tree(trees: &mut Vec<Tree>) -> Option<Tree> {
    if trees.len() == 0 {
        return None;
    }
    let mut lightest_weight = 0;
    let mut lightest_tree_index = 0;
    for (i, t) in trees.iter().enumerate() {
        if lightest_weight > t.weight {
            lightest_weight = t.weight;
            lightest_tree_index = i;
        }
    }

    Some(trees.swap_remove(lightest_tree_index))
}

#[cfg(test)]
mod test {
    use super::encode;

    #[test]
    fn basic() {
        encode(&[1, 2, 1, 2, 3]);
    }
}
