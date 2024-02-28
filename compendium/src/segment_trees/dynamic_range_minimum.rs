use crate::segment_trees::segment_tree::SegmentTree;
use std::cmp::min;
use std::fmt::Debug;
use std::ops::Add;

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

    pub fn add(&mut self, i: usize, v: T) {
        self.segment_tree.update(i, v);
    }

    pub fn rmq(&self, l: usize, r: usize) -> Option<T> {
        self.segment_tree.query(l, r)
    }
}

/// Dynamic Range Minimum Queries with Occurrences.
/// Simply define the combine and propagate functions in the following way:

pub fn min_and_occurrences<T>(a: (T, T), b: (T, T)) -> (T, T)
where
    T: Default + Ord + Add,
{
    return if a.0 == b.0 {
        (a.0, a.1 + b.1)
    } else if a.0 < b.0 {
        a
    } else {
        b
    };
}

/// The propagate function is the same as the combine function. To use the problem just preprocess
/// the array to have couples value, position and build the segment tree on top of it
/// with the combine and propagate functions.

#[test]
pub fn test_dynamic_rmq() {
    let v = vec![1, 2, 3, 4, 5];
    let mut dynamic_rmq = DynamicRMQ::from_vec(v);
    assert_eq!(dynamic_rmq.rmq(0, 4), Some(1));
    dynamic_rmq.add(0, 10);
    assert_eq!(dynamic_rmq.rmq(0, 4), Some(2));
    dynamic_rmq.add(0, 1);
    assert_eq!(dynamic_rmq.rmq(0, 4), Some(2));
}
