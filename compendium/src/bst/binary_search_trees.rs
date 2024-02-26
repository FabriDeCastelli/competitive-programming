use std::fmt::Debug;
use num_traits::{Num, NumCast};

///
pub struct BST<T>
where T: Default + Ord + Debug + Num + NumCast + Copy + Clone
{
    key: T,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>
}

#[derive(PartialEq)]
enum Position {
    LEFT, RIGHT
}

impl<T> BST<T>
where T: Default + Ord + Debug + Num + NumCast + Copy + Clone
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
            right: None
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
    pub fn add(&mut self, node: BST<T>, position: Position)  {

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
            (Some(ref left), Some(ref right)) =>
                1 + left.subtree_size() + right.subtree_size()
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

        return (
            self.key.ge(&max_left) && self.key.le(&min_right) && bst_left && bst_right,
            min_right.min(self.key).min(min_left),
            max_right.max(self.key).max(max_left)
        )

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