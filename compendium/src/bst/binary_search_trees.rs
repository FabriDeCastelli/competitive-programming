use num_traits::{Num, NumCast};
use std::fmt::Debug;

///
pub struct BST<T>
where
    T: Default + Ord + Debug + Num + NumCast + Copy + Clone,
{
    key: T,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>,
}

#[derive(PartialEq)]
pub enum Position {
    LEFT,
    RIGHT,
}

impl<T> BST<T>
where
    T: Default + Ord + Debug + Num + NumCast + Copy + Clone,
{
    /// Creates a new BST<T> with the given key.
    ///
    /// # Arguments
    ///
    /// * `key`: the key of the BST<T>
    ///
    /// returns: BST<T>
    ///
    pub fn new(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }

    /// Adds a new node to the BST<T> at the given position.
    ///
    /// # Arguments
    ///
    /// * `node`: the node to add
    /// * `position`: if left or right child
    ///
    /// returns: ()
    ///
    pub fn add(&mut self, node: BST<T>, position: Position) {
        let node = Box::new(node);
        if position == Position::LEFT {
            self.left = Some(node);
        } else {
            self.right = Some(node);
        }
    }

    /// Returns the size of the subtree rooted at the current node.
    ///
    /// returns: usize
    ///
    pub fn subtree_size(&self) -> usize {
        match (&self.left, &self.right) {
            (None, None) => 1,
            (Some(ref left), None) => 1 + left.subtree_size(),
            (None, Some(ref right)) => 1 + right.subtree_size(),
            (Some(ref left), Some(ref right)) => 1 + left.subtree_size() + right.subtree_size(),
        }
    }

    /// Prints the depth of each node in the BST<T>
    ///
    /// returns: ()
    ///
    pub fn depth(&self) -> () {
        self.depth_rec(0)
    }

    /// Recursive utility function for depth
    ///
    /// # Arguments
    ///
    /// * `d`: the current depth
    ///
    /// returns: ()
    ///
    fn depth_rec(&self, d: usize) -> () {
        match (&self.left, &self.right) {
            (None, None) => {}
            (Some(ref left), None) => left.depth_rec(d + 1),
            (None, Some(ref right)) => right.depth_rec(d + 1),
            (Some(ref left), Some(ref right)) => {
                left.depth_rec(d + 1);
                right.depth_rec(d + 1);
            }
        }
        println!("key: {:?} --- depth: {}", self.key, d);
    }

    /// Checks if the BST<T> is a binary search tree.
    ///
    /// returns: bool
    ///
    pub fn bst_check(&self) -> bool {
        self.bst_check_rec().0
    }

    /// Recursive utility function for bst_check
    ///
    /// returns: (bool, T, T)
    ///
    pub fn bst_check_rec(&self) -> (bool, T, T) {
        let mut bst_left = true;
        let mut min_left = NumCast::from(i32::MAX).unwrap();
        let mut max_left = NumCast::from(i32::MIN).unwrap();
        let mut bst_right = true;
        let mut min_right = NumCast::from(i32::MAX).unwrap();
        let mut max_right = NumCast::from(i32::MIN).unwrap();

        if let Some(ref left) = self.left {
            (bst_left, min_left, max_left) = left.bst_check_rec()
        }

        if let Some(ref right) = self.right {
            (bst_right, min_right, max_right) = right.bst_check_rec()
        }

        (
            self.key.ge(&max_left) && self.key.le(&min_right) && bst_left && bst_right,
            min_right.min(self.key).min(min_left),
            max_right.max(self.key).max(max_left),
        )
    }

    pub fn equally_distanced_nodes(&self) -> usize {
        self.equally_distanced_nodes_rec(T::default()).0
    }

    fn equally_distanced_nodes_rec(&self, distance: T) -> (usize, T) {
        let mut sum_left = T::default();
        let mut count_left = 0;
        let mut sum_right = T::default();
        let mut count_right = 0;

        if let Some(ref left) = self.left {
            (count_left, sum_left) = left.equally_distanced_nodes_rec(distance + self.key);
        }

        if let Some(ref right) = self.right {
            (count_right, sum_right) = right.equally_distanced_nodes_rec(distance + self.key);
        }

        let subtree_sum = sum_left + sum_right + self.key;

        (
            count_left + count_right + if subtree_sum == distance { 1 } else { 0 },
            subtree_sum,
        )
    }

    /// Returns the maximum path sum between to leaves in the BST<T>
    ///
    /// returns: T
    ///
    pub fn maximum_path_sum(&self) -> T {
        self.maximum_path_sum_rec().0
    }

    /// Recursive utility function for maximum_path_sum
    ///
    /// returns: (T, T)
    ///
    fn maximum_path_sum_rec(&self) -> (T, T) {
        let min = NumCast::from(i32::MIN).unwrap();

        let mut best_left = min;
        let mut max_to_left_leave = min;
        let mut best_right = min;
        let mut max_to_right_leaf = min;
        let mut best_current = min;
        let mut max_current = min;

        if let Some(ref left) = self.left {
            (best_left, max_to_left_leave) = left.maximum_path_sum_rec();
        }

        if let Some(ref right) = self.right {
            (best_right, max_to_right_leaf) = right.maximum_path_sum_rec();
        }

        // Weird stuff to avoid overflows in summing min

        if max_to_left_leave == min || max_to_right_leaf == min {
            best_current = best_right.max(best_left);
        } else {
            best_current = best_right
                .max(best_left)
                .max(max_to_right_leaf + max_to_left_leave + self.key);
        }

        max_current = max_to_left_leave.max(max_to_right_leaf);

        // Weird stuff to avoid overflows in summing min (again)

        if max_current == min {
            max_current = T::default();
        }

        max_current = max_current + self.key;

        (best_current, max_current)
    }

    /// Computes the predecessor of a given key in the BST<T>
    /// The key might not belong to the BST<T>
    ///
    /// # Arguments
    ///
    /// * `x`: the key to find the predecessor of
    ///
    /// returns: Option<T>
    ///
    pub fn predecessor(&self, x: T) -> Option<T> {
        if !self.bst_check() {
            println!("The tree is not a BST... can't find predecessor");
            return None;
        }
        self.predecessor_rec(x, None)
    }

    /// Recursive utility function for predecessor
    ///
    /// # Arguments
    ///
    /// * `x`: the key to find the predecessor of
    /// * `predecessor`: the current predecessor
    ///
    /// returns: Option<T>
    ///
    fn predecessor_rec(&self, x: T, mut predecessor: Option<T>) -> Option<T> {
        if self.key.ge(&x) {
            if let Some(ref left) = self.left {
                return left.predecessor_rec(x, predecessor);
            }
        } else {
            predecessor = Some(self.key);
            if let Some(ref right) = self.right {
                return right.predecessor_rec(x, predecessor);
            }
        }
        predecessor
    }

    pub fn successor(&self, x: T) -> Option<T> {
        if !self.bst_check() {
            println!("The tree is not a BST... can't find successor");
            return None;
        }
        self.successor_rec(x, None)
    }

    fn successor_rec(&self, x: T, mut successor: Option<T>) -> Option<T> {

        if self.key.le(&x) {
            if let Some(ref right) = self.right {
                return right.successor_rec(x, successor);
            }
        } else {
            successor = Some(self.key);
            if let Some(ref left) = self.left {
                return left.successor_rec(x, successor);
            }
        }
        successor

    }
}

/// --------- TESTS ------------

fn setup() -> BST<i32> {
    let mut bst = BST::new(2);
    bst.add(BST::new(1), Position::LEFT);
    bst.add(BST::new(3), Position::RIGHT);
    bst
}

#[test]
pub fn test_subtree_size() {
    let bst = setup();

    assert_eq!(bst.subtree_size(), 3);
}

#[test]
pub fn test_depth() {
    let bst = setup();
    bst.depth();
}

#[test]
pub fn test_bst_check() {
    let mut bst = BST::new(2);
    bst.add(BST::new(1), Position::LEFT);
    bst.add(BST::new(3), Position::RIGHT);

    assert_eq!(bst.bst_check(), true);
    let mut bst = BST::new(2);
    bst.add(BST::new(3), Position::LEFT);
    bst.add(BST::new(1), Position::RIGHT);

    assert_eq!(bst.bst_check(), false);
    let mut bst = BST::new(2);
    bst.add(BST::new(1), Position::LEFT);
    bst.add(BST::new(2), Position::RIGHT);
    assert_eq!(bst.bst_check(), true);

    let mut bst = BST::new(2);
    bst.add(BST::new(1), Position::LEFT);
    let mut bst_right = BST::new(3);
    bst_right.add(BST::new(4), Position::RIGHT);
    bst.add(bst_right, Position::RIGHT);
    assert_eq!(bst.bst_check(), true);

    let mut bst = BST::new(5);
    let mut bst_left = BST::new(3);
    bst_left.add(BST::new(2), Position::LEFT);
    bst_left.add(BST::new(4), Position::RIGHT);
    bst.add(bst_left, Position::LEFT);
    bst.add(BST::new(6), Position::RIGHT);
    assert_eq!(bst.bst_check(), true);

    let mut bst = BST::new(5);
    let mut bst_left = BST::new(3);
    bst_left.add(BST::new(2), Position::LEFT);
    bst_left.add(BST::new(6), Position::RIGHT);
    bst.add(bst_left, Position::LEFT);
    bst.add(BST::new(7), Position::RIGHT);
    assert_eq!(bst.bst_check(), false);
}

#[test]
pub fn test_equally_distanced_nodes() {
    let mut bst = BST::new(1);
    let mut bst_left = BST::new(1);
    bst_left.add(BST::new(2), Position::LEFT);
    bst.add(bst_left, Position::LEFT);
    assert_eq!(bst.equally_distanced_nodes(), 1);

    let mut bst = BST::new(0);
    bst.add(BST::new(0), Position::LEFT);
    bst.add(BST::new(0), Position::RIGHT);
    assert_eq!(bst.equally_distanced_nodes(), 3);

    let mut bst = BST::new(2);
    bst.add(BST::new(1), Position::LEFT);
    bst.add(BST::new(3), Position::RIGHT);
    assert_eq!(bst.equally_distanced_nodes(), 0);
}

#[test]
pub fn test_maximum_path_sum() {
    let mut bst = BST::new(1);
    let mut bst_left = BST::new(2);
    bst_left.add(BST::new(4), Position::LEFT);
    bst_left.add(BST::new(5), Position::RIGHT);
    bst.add(bst_left, Position::LEFT);
    let mut bst_right = BST::new(3);
    bst_right.add(BST::new(6), Position::LEFT);
    bst_right.add(BST::new(7), Position::RIGHT);
    bst.add(bst_right, Position::RIGHT);
    assert_eq!(bst.maximum_path_sum(), 18);
}

#[test]
pub fn test_predecessor_successor() {
    let mut bst = BST::new(2);
    bst.add(BST::new(1), Position::LEFT);
    bst.add(BST::new(3), Position::RIGHT);
    // Predecessor
    assert_eq!(bst.predecessor(3), Some(2));
    assert_eq!(bst.predecessor(1), None);
    assert_eq!(bst.predecessor(2), Some(1));
    assert_eq!(bst.predecessor(4), Some(3));

    // Successor
    assert_eq!(bst.successor(3), None);
    assert_eq!(bst.successor(1), Some(2));
    assert_eq!(bst.successor(2), Some(3));
    assert_eq!(bst.successor(-10), Some(1));


}
