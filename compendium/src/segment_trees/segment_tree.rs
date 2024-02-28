use std::fmt::Debug;

struct SegmentTree<T, F>
where
    T: Default + Copy + Clone + Debug,
    F: Fn(T, T) -> T,
{
    length: usize,
    tree: Vec<T>,
    lazy: Vec<T>,
    propagate: F,
    combine: F,
}

impl<T, F> SegmentTree<T, F>
where
    T: Default + Copy + Clone + Debug,
    F: Fn(T, T) -> T,
{
    pub fn from_vec(v: Vec<T>, propagate: F, combine: F) -> Self {
        let length = v.len();
        let lazy = vec![T::default(); 4 * length];
        let tree = vec![T::default(); 4 * length];
        let mut segment_tree = Self {
            length,
            tree,
            lazy,
            propagate,
            combine,
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

    fn left_child(index: usize) -> usize {
        2 * index + 1
    }

    fn right_child(index: usize) -> usize {
        2 * index + 2
    }
}
