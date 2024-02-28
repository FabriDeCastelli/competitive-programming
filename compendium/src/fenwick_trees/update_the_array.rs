use std::ops::{Add, Neg, Sub};
use crate::fenwick_trees::fenwick_tree::FenwickTree;

struct UpdateArray<T>
where T:
    Default + Clone + Copy + Add<Output = T> + Sub<Output = T> + Neg<Output = T>
{
    fenwick_tree: FenwickTree<T>,
}


impl <T> UpdateArray<T>
where T:
    Default + Clone + Copy + Add<Output = T> + Sub<Output = T> + Neg<Output = T>
{
    pub fn with_capacity(n: usize) -> Self {
        Self {
            fenwick_tree: FenwickTree::with_capacity(n)
        }
    }

    /// Updates the array with a value v in the range [l, r].
    ///
    /// # Arguments
    ///
    /// * `l`: the left endpoint of the range
    /// * `r`: the right endpoint of the range
    /// * `v`: the value to be added to the range
    ///
    /// returns: ()
    /// \theta(log n)
    pub fn update_range(&mut self, l: usize, r: usize, v: T) {
        self.fenwick_tree.add(l, v);
        self.fenwick_tree.add(r + 1, -v);
    }

    /// Access the value at position i.
    ///
    /// # Arguments
    ///
    /// * `i`: the position in the array
    ///
    /// returns: T
    ///
    /// \theta(log n)
    pub fn access(&self, i: usize) -> T {
        self.fenwick_tree.sum(i)
    }


}


#[test]
pub fn test_update_access() {
    let mut a = UpdateArray::with_capacity(8);
    a.update_range(3, 5, 10);
    assert_eq!(a.access(4), 10);
}