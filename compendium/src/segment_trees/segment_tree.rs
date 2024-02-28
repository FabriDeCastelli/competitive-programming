use std::fmt::Debug;
use std::ops::Mul;

pub struct SegmentTree<T, F>
where
    T: Default + Copy + Clone + Debug + Ord + From<usize> + Mul<T, Output = T>,
    F: Fn(T, T) -> T,
{
    length: usize,
    tree: Vec<T>,
    lazy: Vec<T>,
    propagate: F,
    combine: F,
    update: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Default + Copy + Clone + Debug + Ord + From<usize> + Mul<T, Output = T>,
    F: Fn(T, T) -> T,
{
    pub fn from_vec(v: Vec<T>, propagate: F, combine: F, update: F) -> Self {
        let length = v.len();
        let lazy = vec![T::default(); length + length + length + length];
        let tree = vec![T::default(); length + length + length + length];
        let mut segment_tree = Self {
            length,
            tree,
            lazy,
            propagate,
            combine,
            update,
        };
        segment_tree.build(v.as_slice(), 0, length - 1, 0);
        segment_tree
    }

    fn build(&mut self, data: &[T], l: usize, r: usize, current: usize) {
        // leaf node
        if l == r {
            self.tree[current] = data[l];
            return;
        }

        let mid = (l + r) / 2;
        let left_child = Self::left_child(current);
        let right_child = Self::right_child(current);

        self.build(data, l, mid, left_child);
        self.build(data, mid + 1, r, right_child);

        self.tree[current] = (self.combine)(self.tree[left_child], self.tree[right_child]);
    }

    /// Update the value at index `i` to `v`.
    ///
    /// # Arguments
    ///
    /// * `i`: the index to update
    /// * `v`: the value to update to
    ///
    /// returns: ()
    /// \theta(\log n)
    pub fn update(&mut self, i: usize, v: T) {
        self.update_rec(i, v, 0, self.length - 1, 0)
    }

    fn update_rec(&mut self, i: usize, v: T, l: usize, r: usize, current: usize) {
        // leaf node
        if l == r {
            self.tree[current] = (self.update)(self.tree[current], v);
            return;
        }

        let mid = (l + r) / 2;
        let left_child = Self::left_child(current);
        let right_child = Self::right_child(current);

        // decide which subtree to go
        if i <= mid {
            self.update_rec(i, v, l, mid, left_child);
        } else {
            self.update_rec(i, v, mid + 1, r, right_child);
        }

        // we use combine because we update the leaf node only and then propagate to the root
        self.tree[current] = (self.combine)(self.tree[left_child], self.tree[right_child]);
    }

    /// Performs a range update on the segment tree.
    ///
    /// # Arguments
    ///
    /// * `ql`: the left endpoint of the range
    /// * `qr`: the right endpoint of the range
    /// * `v`: the value to update to
    ///
    /// returns: ()
    /// \theta(\log n)
    pub fn range_update(&mut self, ql: usize, qr: usize, v: T) {
        self.range_update_rec(ql, qr, v, 0, self.length - 1, 0);
    }

    pub fn range_update_rec(
        &mut self,
        ql: usize,
        qr: usize,
        v: T,
        l: usize,
        r: usize,
        current: usize,
    ) {
        // handle lazy propagation
        self.lazy_propagate(l, r, current);

        if Self::no_overlap(ql, qr, l, r) {
            return;
        }

        let left_child = Self::left_child(current);
        let right_child = Self::right_child(current);

        if Self::total_overlap(ql, qr, l, r) {
            self.tree[current] = (self.update)(self.tree[current], T::from(r - l + 1) * v);

            // if not leaf node then set the lazy value for children nodes
            if l != r {
                self.lazy[Self::left_child(current)] = (self.update)(self.lazy[left_child], v);
                self.lazy[Self::right_child(current)] = (self.update)(self.lazy[right_child], v);
            }

            return;
        }

        let mid = (l + r) / 2;
        self.range_update_rec(ql, qr, v, l, mid, left_child);
        self.range_update_rec(ql, qr, v, mid + 1, r, right_child);

        self.tree[current] = (self.combine)(self.tree[left_child], self.tree[right_child]);
    }

    pub fn print(&self) {
        self.print_recursive(0, 0, self.length - 1);
    }

    fn print_recursive(&self, pos: usize, left: usize, right: usize) {
        println!(
            "Node: {}, Range: [{}, {}], Value: {:?}, Lazy: {:?}",
            pos, left, right, self.tree[pos], self.lazy[pos]
        );

        if left != right {
            let mid = (left + right) / 2;
            self.print_recursive(Self::left_child(pos), left, mid);
            self.print_recursive(Self::right_child(pos), mid + 1, right);
        }
    }

    pub fn query(&mut self, ql: usize, qr: usize) -> Option<T> {
        assert!(ql <= qr);
        self.query_recursive(ql, qr, 0, self.length - 1, 0)
    }

    fn query_recursive(
        &mut self,
        ql: usize,
        qr: usize,
        l: usize,
        r: usize,
        current: usize,
    ) -> Option<T> {
        self.lazy_propagate(l, r, current);

        if Self::no_overlap(ql, qr, l, r) {
            return None;
        }
        if Self::total_overlap(ql, qr, l, r) {
            return Some(self.tree[current]);
        }

        let mid = (l + r) / 2;
        let left = self.query_recursive(ql, qr, l, mid, Self::left_child(current));
        let right = self.query_recursive(ql, qr, mid + 1, r, Self::right_child(current));

        match (left, right) {
            (Some(l), Some(r)) => Some((self.propagate)(l, r)),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            _ => None,
        }
    }

    fn lazy_propagate(&mut self, l: usize, r: usize, current: usize) {
        let left_child = Self::left_child(current);
        let right_child = Self::right_child(current);
        if self.lazy[current] != T::default() {
            self.tree[current] =
                (self.update)(self.tree[current], T::from(r - l + 1) * self.lazy[current]);

            if l != r {
                self.lazy[left_child] = (self.update)(self.lazy[left_child], self.lazy[current]);
                self.lazy[right_child] = (self.update)(self.lazy[right_child], self.lazy[current]);
            }

            self.lazy[current] = T::default();
        }
    }

    fn no_overlap(ql: usize, qr: usize, l: usize, r: usize) -> bool {
        qr < l || r < ql
    }

    fn total_overlap(ql: usize, qr: usize, l: usize, r: usize) -> bool {
        ql <= l && r <= qr
    }

    fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    fn right_child(index: usize) -> usize {
        2 * index + 2
    }
}

#[test]
fn test_segment_tree() {
    let v = vec![1, 2, 3, 4];
    let sum = |a, b| a + b;
    let mut segment_tree = SegmentTree::from_vec(v, sum, sum, sum);
    segment_tree.update(0, 3);
    assert_eq!(segment_tree.query(0, 3), Some(13));
}

#[test]
fn test_segment_tree_range_update() {
    let v = vec![1, 1, 1, 1];
    let sum = |a, b| a + b;
    let mut segment_tree = SegmentTree::from_vec(v, sum, sum, sum);
    segment_tree.range_update(0, 2, 2);
    assert_eq!(segment_tree.query(0, 3), Some(10));
    assert_eq!(segment_tree.query(0, 0), Some(3));
}
