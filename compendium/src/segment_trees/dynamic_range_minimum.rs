use std::cmp::min;
use std::fmt::Debug;
use std::ops::Add;
use crate::segment_trees::segment_tree::SegmentTree;

// Assume SegmentTree is defined somewhere

struct DynamicRMQ<T>
    where
        T: Default + Copy + Clone + Debug + Add<Output = T> + PartialOrd,
{
    segment_tree: SegmentTree<T, fn(T, T) -> T>,
}

impl<T> DynamicRMQ<T>
    where
        T: Default + Copy + Clone + Debug + Add<Output = T> + PartialOrd + Ord,
{
    pub fn from_vec(v: Vec<T>) -> Self {
        Self {
            segment_tree: SegmentTree::from_vec(v, min, min, |a, b| a + b),
        }
    }

    pub fn update(&mut self, i: usize, v: T) {
        self.segment_tree.update(i, v);
    }

    pub fn query(&self, l: usize, r: usize) -> Option<T> {
        self.segment_tree.query(l, r)
    }

}

#[test]
pub fn test_dynamic_rmq() {
    let v = vec![1, 2, 3, 4, 5];
    let mut dynamic_rmq = DynamicRMQ::from_vec(v);
    assert_eq!(dynamic_rmq.query(0, 4), Some(1));
    dynamic_rmq.update(0, 10);
    assert_eq!(dynamic_rmq.query(0, 4), Some(2));
    dynamic_rmq.update(0, 1);
    assert_eq!(dynamic_rmq.query(0, 4), Some(2));
}
