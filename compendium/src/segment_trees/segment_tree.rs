use std::fmt::Debug;

pub struct SegmentTree<T, F>
where
    T: Default + Copy + Clone + Debug,
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
    T: Default + Copy + Clone + Debug,
    F: Fn(T, T) -> T,
{
    pub fn from_vec(v: Vec<T>, propagate: F, combine: F, update: F) -> Self {
        let length = v.len();
        let lazy = vec![T::default(); 4 * length];
        let tree = vec![T::default(); 4 * length];
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

    pub fn update(&mut self, i: usize, v: T) {
        self.update_rec(i, v, 0, self.length - 1, 0)
    }

    fn update_rec(&mut self, i: usize, v: T, l: usize, r: usize, current: usize) {
        if l == r {
            self.tree[current] = (self.update)(self.tree[current], v);
            return;
        }

        let mid = (l + r) / 2;
        let left_child = Self::left_child(current);
        let right_child = Self::right_child(current);

        if i <= mid {
            self.update_rec(i, v, l, mid, left_child);
        } else {
            self.update_rec(i, v, mid + 1, r, right_child);
        }

        // we use combine because we update the leaf node only and then propagate to the root
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

    pub fn query(&self, ql: usize, qr: usize) -> Option<T> {
        assert!(ql <= qr);
        self.query_recursive(ql, qr, 0, self.length - 1, 0)
    }

    fn query_recursive(
        &self,
        ql: usize,
        qr: usize,
        l: usize,
        r: usize,
        current: usize,
    ) -> Option<T> {
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
    segment_tree.print();
    assert_eq!(segment_tree.query(0, 3), Some(13));
}
