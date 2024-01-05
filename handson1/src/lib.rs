use std::cmp::{max, min};
use std::u32;

struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

struct Tree {
    nodes: Vec<Node>,
}

/// This a representation of a tree.
/// Every node has an implicit id, which is its position on the vector `nodes`.
/// Every node has a key and at most two children. The ids of the children are
/// stored in `id_left` and `id_right`. These ids are `None` iff the child does not exit.
impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left child of the node `parent_id`
    /// iff `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /** --------------------- Problem #1 ---------------------**/
    pub fn check_bst(&self) -> bool {
        self.rec_check_bst(Some(0)).0
    }

    fn rec_check_bst(&self, node_id: Option<usize>) -> (bool, u32, u32) {
        if let Some(id) = node_id {
            let node = &self.nodes[id];
            let (bl, min_l, max_l) = self.rec_check_bst(node.id_left);
            let (br, min_r, max_r) = self.rec_check_bst(node.id_right);
            let min = min(min(node.key, min_l), min_r);
            let max = max(max(node.key, max_l), max_r);
            if max_l > node.key || min_r <= node.key {
                return (false, min, max);
            }
            return (bl && br, min, max);
        }
        (true, u32::MAX, 0)
    }

    /** --------------------- Problem #2 ---------------------**/
    pub fn check_balanced(&self) -> bool {
        self.rec_check_balanced(Some(0)).0
    }

    fn rec_check_balanced(&self, node_id: Option<usize>) -> (bool, u32) {
        if let Some(id) = node_id {
            let node = &self.nodes[id];
            let (check_left, height_left) = self.rec_check_balanced(node.id_left);
            let (check_right, height_right) = self.rec_check_balanced(node.id_right);
            let check = check_left && check_right && (2 > height_left.abs_diff(height_right));
            return (check, max(height_left, height_right) + 1);
        }
        (true, 0)
    }

    /** --------------------- Problem #3 ---------------------**/
    pub fn check_max_heap(&self) -> bool {
        // the size of the tree (the length of the array of nodes) could be computed with self.nodes.len()
        // to remain aligned with problems, I decided to compute it manually with recursion
        let n = self.count_nodes(Some(0));
        self.check_complete(Some(0), 0, n) && self.check_max_heap_property(Some(0)).0
    }

    fn count_nodes(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            let node = &self.nodes[id];
            return 1 + self.count_nodes(node.id_left) + self.count_nodes(node.id_right);
        }
        0
    }

    fn check_complete(&self, node_id: Option<usize>, index: u32, num_elem: u32) -> bool {
        if let Some(id) = node_id {
            if index >= num_elem {
                return false;
            }
            let node = &self.nodes[id];
            let cl = self.check_complete(node.id_left, 2 * index + 1, num_elem);
            let cr = self.check_complete(node.id_right, 2 * index + 2, num_elem);
            return cl && cr;
        }
        true
    }

    fn check_max_heap_property(&self, node_id: Option<usize>) -> (bool, u32) {
        if let Some(id) = node_id {
            let node = &self.nodes[id];
            let (mhl, ml) = self.check_max_heap_property(node.id_left);
            let (mhr, mr) = self.check_max_heap_property(node.id_right);
            return (mhl && mhr && node.key >= max(ml, mr), node.key);
        }
        (true, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);

        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert_eq!(tree.sum(), 64);
    }

    /// Tests for the problem #1
    #[test]
    fn test_check_bst() {
        // Test cases where trees are not BSTs
        let mut not_bst_tree = Tree::with_root(10);
        not_bst_tree.add_node(0, 11, true);
        assert!(
            !not_bst_tree.check_bst(),
            "The tree should not be a BST, but it is!"
        );

        let mut another_not_bst = Tree::with_root(10);
        another_not_bst.add_node(0, 5, true);
        another_not_bst.add_node(1, 12, false);
        another_not_bst.add_node(1, 7, true);
        another_not_bst.add_node(0, 18, false);
        assert!(
            !another_not_bst.check_bst(),
            "Node with key 12 should break BST property!"
        );

        // Test cases where trees are BSTs
        let mut bst_tree = Tree::with_root(2);
        assert!(bst_tree.check_bst(), "Single node tree should be a BST");

        bst_tree.add_node(0, 2, true); // ID 1
        bst_tree.add_node(1, 1, true); // ID 2
        bst_tree.add_node(0, 4, false); // ID 3
        bst_tree.add_node(3, 3, true); // ID 4
        bst_tree.add_node(3, 5, false); // ID 5

        assert!(bst_tree.check_bst(), "This should be a BST");
    }

    /// Tests for the problem #2
    #[test]
    fn test_check_balanced() {
        // Test cases where trees are balanced
        let mut balanced = Tree::with_root(0);
        balanced.add_node(0, 0, true); // ID 1
        balanced.add_node(0, 0, false); // ID 2
        balanced.add_node(1, 0, true); // ID 3
        balanced.add_node(1, 0, false); // ID 4
        assert!(balanced.check_balanced(), "Tree should be balanced");

        let _only_root_tree = Tree::with_root(0);
        assert!(
            _only_root_tree.check_balanced(),
            "A tree with only the root should be balanced"
        );

        // Test cases where trees are not balanced
        let mut not_balanced = Tree::with_root(0);
        not_balanced.add_node(0, 1, true); // ID 1
        not_balanced.add_node(1, 2, false); // ID 2
        assert!(
            !not_balanced.check_balanced(),
            "Tree should not be balanced"
        );

        let mut another_unbalanced_tree = Tree::with_root(0);
        another_unbalanced_tree.add_node(0, 0, true); // ID 1
        another_unbalanced_tree.add_node(0, 0, false); // ID 2
        another_unbalanced_tree.add_node(1, 0, true); // ID 3
        another_unbalanced_tree.add_node(1, 0, false); // ID 4
        another_unbalanced_tree.add_node(3, 0, true); // ID 5
        assert!(
            !another_unbalanced_tree.check_balanced(),
            "This tree should not be balanced as \
        the left subtree has height 3 and the right has height 1"
        );
    }

    /// Tests for the problem #3
    #[test]
    pub fn test_max_heap() {
        // Test cases where trees are max heaps
        let mut max_heap_tree = Tree::with_root(25);
        max_heap_tree.add_node(0, 10, true); // ID 1
        max_heap_tree.add_node(0, 8, false); // ID 2
        max_heap_tree.add_node(1, 3, true); // ID 3
        max_heap_tree.add_node(1, 5, false); // ID 4
        max_heap_tree.add_node(2, 7, true); // ID 5
        assert!(
            max_heap_tree.check_max_heap(),
            "The tree should be a max heap"
        );

        let only_root = Tree::with_root(0);
        assert!(
            only_root.check_max_heap(),
            "A tree with only the root should be a max heap"
        );

        // Test case where the tree is not even complete
        let mut not_complete_tree = Tree::with_root(25);
        not_complete_tree.add_node(0, 10, true); // ID 1
        not_complete_tree.add_node(0, 8, false); // ID 2
        not_complete_tree.add_node(1, 3, true); // ID 3
        not_complete_tree.add_node(1, 5, false); // ID 4
        not_complete_tree.add_node(2, 7, false); // ID 5
        assert!(
            !not_complete_tree.check_max_heap(),
            "The tree should not be a max heap"
        );

        // Test case where the tree does not satisfy the max heap property
        let mut not_max_heap_tree = Tree::with_root(2);
        not_max_heap_tree.add_node(0, 2, true);
        not_max_heap_tree.add_node(0, 3, false);
        assert!(
            !not_complete_tree.check_max_heap(),
            "The tree should not be a max heap"
        );

        // Test case where the tree does not satisfy both properties
        let mut not_max_heap_and_complete = Tree::with_root(34);
        not_max_heap_and_complete.add_node(0, 35, false);
        assert!(
            !not_complete_tree.check_max_heap(),
            "The tree should not be a max heap: it doesn't satisfy bot properties"
        );
    }
}
