use crate::fenwick_trees::fenwick_tree::FenwickTree;
use std::ops::{Add, Mul, Neg, Sub};

struct UpdateArray<T>
where
    T: Default
        + Clone
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + PartialOrd
        + PartialEq
        + From<i32>,
{
    fenwick_tree: FenwickTree<T>,
    correction_tree: FenwickTree<T>,
}

impl<T> UpdateArray<T>
where
    T: Default
        + Clone
        + Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + PartialOrd
        + PartialEq
        + From<i32>,
{
    pub fn with_capacity(n: usize) -> Self {
        Self {
            fenwick_tree: FenwickTree::with_capacity(n + 1),
            correction_tree: FenwickTree::with_capacity(n + 1),
        }
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.fenwick_tree.len(), self.correction_tree.len());
        self.fenwick_tree.len()
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
        self.correction_tree.add(l, -v * T::from(l as i32 - 1));
        if r + 1 < self.len() {
            self.fenwick_tree.add(r + 1, -v);
            self.correction_tree.add(r + 1, v * T::from(r as i32));
        }
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
        self.sum(i)
            - if i == 0 {
                T::default()
            } else {
                self.sum(i - 1)
            }
    }

    pub fn sum(&self, i: usize) -> T {
        self.fenwick_tree.sum(i) * T::from(i as i32) + self.correction_tree.sum(i)
    }

    pub fn range_sum(&self, l: usize, r: usize) -> T {
        self.sum(r) - self.sum(l - 1)
    }
}

#[test]
pub fn test_update_access() {
    let mut a = UpdateArray::with_capacity(8);
    a.update_range(3, 5, 10);
    assert_eq!(a.access(4), 10);
    assert_eq!(a.access(5), 10);
    assert_eq!(a.access(6), 0);
    assert_eq!(a.range_sum(3, 5), 30);
    assert_eq!(a.range_sum(4, 5), 20);

}
