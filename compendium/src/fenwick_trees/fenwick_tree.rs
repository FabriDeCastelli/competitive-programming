use std::ops::Add;

pub struct FenwickTree<T>
where
    T: Default + Clone + Copy + Add<Output = T>,
{
    tree: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Default + Clone + Copy + Add<Output = T>,
{
    /// Create an empty FenwickTree with a given capacity.
    /// The tree has n + 1 elements, the first element is always the null element.
    ///
    /// # Arguments
    ///
    /// * `n`: the capacity of the tree
    ///
    /// returns: FenwickTree<T>
    ///
    pub fn with_capacity(n: usize) -> Self {
        Self {
            tree: vec![T::default(); n + 1],
        }
    }

    /// Create a FenwickTree from a vector.
    /// The first element of the tree (root) is always the null element.
    ///
    /// # Arguments
    ///
    /// * `v`: the input vector
    ///
    /// returns: FenwickTree<T>
    /// \theta(n log n)
    pub fn from_vec(v: Vec<T>) -> Self {
        let n = v.len();
        let mut fenwick_tree = Self::with_capacity(n + 1);

        for i in 1..=n {
            fenwick_tree.add(i, v[i - 1]);
        }

        fenwick_tree
    }

    /// Compute the sum of the first i elements of the tree.
    ///
    /// # Arguments
    ///
    /// * `i`: the position in the tree
    ///
    /// returns: T
    /// \theta(log n)
    pub fn sum(&self, mut i: usize) -> T {
        let mut sum = T::default();
        while i != 0 && i < self.tree.len() {
            sum = sum + self.tree[i];
            i = Self::parent(i);
        }
        sum
    }

    /// Add a value to the tree at position i.
    ///
    /// # Arguments
    ///
    /// * `i`: the position in the tree
    /// * `v`: the value to add
    ///
    /// returns: ()
    /// \theta(log n)
    pub fn add(&mut self, mut i: usize, v: T) {
        while i != 0 && i < self.tree.len() {
            self.tree[i] = self.tree[i] + v;
            i = Self::move_right(i);
        }
    }

    /// Compute the parent of a node in the tree.
    ///
    /// # Arguments
    ///
    /// * `i`: the position in the tree
    ///
    /// returns: usize
    /// \theta(1)
    fn parent(i: usize) -> usize {
        let j = i as i32;
        (j - (j & -j)) as usize
    }

    /// Compute the next node in the tree.
    ///
    /// # Arguments
    ///
    /// * `i`: the position in the tree
    ///
    /// returns: usize
    /// \theta(1)
    fn move_right(i: usize) -> usize {
        let j = i as i32;
        (j + (j & -j)) as usize
    }
}

#[test]
pub fn test_fenwick_tree() {
    let a = vec![1, 2, 3, 4, 5];
    let mut fenwick_tree = FenwickTree::from_vec(a);
    assert_eq!(fenwick_tree.sum(5), 15);
    assert_eq!(fenwick_tree.sum(4), 10);
    assert_eq!(fenwick_tree.sum(3), 6);
    assert_eq!(fenwick_tree.sum(2), 3);
    assert_eq!(fenwick_tree.sum(1), 1);
    fenwick_tree.add(3, 2);
    assert_eq!(fenwick_tree.sum(5), 17);
    assert_eq!(fenwick_tree.sum(4), 12);
    assert_eq!(fenwick_tree.sum(3), 8);
    assert_eq!(fenwick_tree.sum(2), 3);
    assert_eq!(fenwick_tree.sum(1), 1);
}
