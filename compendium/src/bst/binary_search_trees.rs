use crate::utilities::MinMax;
use std::fmt::Debug;
use std::ops::Add;

#[derive(Clone)]
pub struct BST<T>
where
    T: Default + Ord + Debug + MinMax + Copy + Clone,
{
    key: T,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>,
}

impl<T> BST<T>
where
    T: Default + Ord + Debug + MinMax + Copy + Clone,
{
    /// Creates a new BST<T> with the given key.
    ///
    /// # Arguments
    ///
    /// * `key`: the key of the BST<T>
    ///
    /// returns: BST<T>
    ///
    pub fn with_root(key: T) -> Self {
        Self {
            key,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        let mut bst = BST::with_root(v[0]);
        for i in 1..v.len() {
            bst.add(v[i]);
        }
        bst
    }

    pub fn print(&self) {
        println!("{:?}", self.key);
        if let Some(ref left) = self.left {
            left.print();
        }
        if let Some(ref right) = self.right {
            right.print();
        }
    }

    /// Adds a new node to the tree maintaining the BST property.
    ///
    /// # Arguments
    ///
    /// * `val`: the value to add to the BST
    ///
    /// returns: ()
    ///
    pub fn add(&mut self, val: T) {
        if val < self.key {
            if let Some(ref mut left) = self.left {
                left.add(val);
            } else {
                self.left = Some(Box::new(BST::with_root(val)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.add(val);
            } else {
                self.right = Some(Box::new(BST::with_root(val)));
            }
        }
    }

    pub fn min(&self) -> &Self {
        if let Some(ref left) = self.left {
            return left.min();
        }
        self
    }

    pub fn max(&self) -> &Self {
        if let Some(ref right) = self.right {
            return right.max();
        }
        self
    }

    pub fn delete(&mut self, x: T) -> bool {
        if !self.search(x) {
            return false;
        }
        match self.delete_rec(x) {
            None => false,
            Some(_) => true,
        }
    }

    fn delete_rec(&mut self, x: T) -> Option<Box<BST<T>>> {
        if self.key.eq(&x) {
            // Case 1: no children
            if self.left.is_none() && self.right.is_none() {
                return None;
            }
            // Case 2: only left child
            if self.right.is_none() {
                let mut left_subtree = self.left.take().unwrap();
                // take maximum of the left subtree
                let max = left_subtree.max();
                // set current node value as the maximum
                self.key = max.key;
                // delete the maximum recursively
                self.left = left_subtree.delete_rec(max.key);
            } else {
                // Case 3, 4: both children or only right child
                let mut right_subtree = self.right.take().unwrap();
                // take minimum of the right subtree
                let min = right_subtree.min();
                // set current node value as the minimum
                self.key = min.key;
                // delete the minimum recursively
                self.right = right_subtree.delete_rec(min.key);
            }
        } else if self.key.lt(&x) {
            if let Some(right) = self.right.as_mut() {
                let new_right = right.delete_rec(x);
                self.right = new_right;
            }
        } else {
            if let Some(left) = self.left.as_mut() {
                let new_left = left.delete_rec(x);
                self.left = new_left;
            }
        }

        Some(Box::new(self.clone()))
    }

    pub fn update(&mut self, key: T, val: T) -> bool {
        if !self.delete(key) {
            return false;
        }

        self.add(val);
        true
    }

    pub fn get_leaves(&self) -> Vec<T> {
        let mut leaves = Vec::new();
        self.get_leaves_rec(&mut leaves);
        leaves.iter().for_each(|x| println!("{:?}", x));
        leaves
    }

    fn get_leaves_rec(&self, leaves: &mut Vec<T>) {
        if self.left.is_none() && self.right.is_none() {
            leaves.push(self.key);
        }
        if let Some(ref left) = self.left {
            left.get_leaves_rec(leaves);
        }
        if let Some(ref right) = self.right {
            right.get_leaves_rec(leaves);
        }
    }

    pub fn search(&self, x: T) -> bool {
        if self.key.eq(&x) {
            return true;
        }
        if self.key.lt(&x) {
            if let Some(ref right) = self.right {
                return right.search(x);
            }
        } else {
            if let Some(ref left) = self.left {
                return left.search(x);
            }
        }
        false
    }

    /// Returns the size of the tree.
    ///
    /// returns: usize
    ///
    pub fn size(&self) -> usize {
        match (&self.left, &self.right) {
            (None, None) => 1,
            (Some(ref left), None) => 1 + left.size(),
            (None, Some(ref right)) => 1 + right.size(),
            (Some(ref left), Some(ref right)) => 1 + left.size() + right.size(),
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
        let mut min_left = T::MAX;
        let mut max_left = T::MIN;
        let mut bst_right = true;
        let mut min_right = T::MAX;
        let mut max_right = T::MIN;

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
}

impl<T> BST<T>
where
    T: Default + Ord + Debug + MinMax + Copy + Clone + Add<Output = T>,
{
    pub fn equally_distanced_nodes(&self) -> usize {
        self.equally_distanced_nodes_rec(<T>::default()).0
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
        let min = T::MIN;

        let mut best_left = min;
        let mut max_to_left_leave = min;
        let mut best_right = min;
        let mut max_to_right_leaf = min;

        if let Some(ref left) = self.left {
            (best_left, max_to_left_leave) = left.maximum_path_sum_rec();
        }

        if let Some(ref right) = self.right {
            (best_right, max_to_right_leaf) = right.maximum_path_sum_rec();
        }

        // Weird stuff to avoid overflows in summing min

        let best_current = if max_to_left_leave.eq(&min) || max_to_right_leaf.eq(&min) {
            best_right.max(best_left)
        } else {
            best_right
                .max(best_left)
                .max(max_to_right_leaf + max_to_left_leave + self.key)
        };

        let mut max_current = max_to_left_leave.max(max_to_right_leaf);

        // Weird stuff to avoid overflows in summing min (again)

        if max_current.eq(&min) {
            max_current = T::default();
        }

        max_current = max_current + self.key;

        (best_current, max_current)
    }
}

/// --------- TESTS ------------

fn setup() -> BST<i32> {
    let mut bst = BST::with_root(2);
    bst.add(1);
    bst.add(3);
    bst
}

#[test]
pub fn test_size() {
    let bst = setup();
    assert_eq!(bst.size(), 3);
}

#[test]
pub fn test_depth() {
    let bst = setup();
    bst.depth();
}

#[test]
pub fn test_bst_check() {
    let mut bst = setup();
    assert_eq!(bst.bst_check(), true);

    bst = BST::with_root(2);
    bst.add(3);
    bst.add(1);
    assert_eq!(bst.bst_check(), true);
}

#[test]
pub fn test_equally_distanced_nodes() {
    let mut bst = BST::with_root(1);
    bst.add(1);
    bst.add(2);
    assert_eq!(bst.equally_distanced_nodes(), 1);

    bst = BST::with_root(0);
    bst.add(0);
    bst.add(0);
    assert_eq!(bst.equally_distanced_nodes(), 3);

    bst = setup();
    assert_eq!(bst.equally_distanced_nodes(), 0);
}

#[test]
pub fn test_maximum_path_sum() {
    let mut bst = BST::with_root(4);
    bst.add(2);
    bst.add(6);
    bst.add(1);
    bst.add(3);
    bst.add(5);
    bst.add(7);
    bst.get_leaves();
    assert_eq!(bst.maximum_path_sum(), 22);
}

#[test]
pub fn test_predecessor_successor() {
    let bst = setup();
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

#[test]
pub fn test_min() {
    let mut bst = setup();
    assert_eq!(bst.min().key, 1);
    bst.add(0);
    assert_eq!(bst.min().key, 0);
}

#[test]
pub fn test_delete() {
    let mut bst = setup();
    assert!(bst.search(1));
    assert_eq!(bst.size(), 3);
    assert!(bst.delete(1));
    assert!(!bst.search(1));
    assert_eq!(bst.size(), 2);
}

#[test]
pub fn test_update() {
    let mut bst = setup();
    assert!(bst.search(1));
    assert_eq!(bst.size(), 3);
    assert!(bst.update(1, 10));
    assert!(!bst.search(1));
    assert!(bst.search(10));
    assert_eq!(bst.size(), 3);
}

#[test]
pub fn test_delete_couples() {
    let mut bst = BST::with_root((1, 2));
    bst.add((3, 2));
    bst.add((5, 2));
    bst.add((2, 1));
    bst.add((4, 1));
    bst.add((6, 1));
    assert_eq!(bst.size(), 6);

    assert!(bst.delete((2, 1)));
    assert_eq!(bst.size(), 5);
    bst.print();
    println!("-------------------");

    assert!(bst.delete((3, 2)));
    assert_eq!(bst.size(), 4);
    bst.print();
    println!("-------------------");

    assert!(bst.delete((5, 2)));
    assert_eq!(bst.size(), 3);
    bst.print();
    println!("-------------------");

    println!("Root: {:?}", bst.key);
    if let Some(ref left) = bst.left {
        println!("Left: {:?}", left.key);
    } else {
        println!("Left: None");
    }
    if let Some(ref right) = bst.right {
        println!("Right: {:?}", right.key);
    } else {
        println!("Right: None");
    }
    assert!(bst.delete((1, 2)));
    bst.print();
    println!("-------------------");
    assert_eq!(bst.size(), 2);
}
