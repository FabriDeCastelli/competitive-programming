use crate::fenwick_trees::fenwick_tree::FenwickTree;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub};

/// Given an array of n elements, count the number of position i < j such that a[i] > a[j].
/// Measures the 'unsortedness' of the array.
///
/// # Arguments
///
/// * `a`: the input array
///
/// returns: usize
/// \theta(n log n)
pub fn counting_inversions<T>(mut a: Vec<T>) -> T
where
    T: Ord
        + Clone
        + Add<Output = T>
        + Sub<Output = T>
        + From<u8>
        + Copy
        + Default
        + AddAssign
        + PartialEq
        + Debug
        + Hash, usize: From<T>,
{
    let n = a.len();

    a = remap(a, n);

    let mut fenwick_tree = FenwickTree::with_capacity(n);
    let mut result = T::default();

    for i in 0..n {
        fenwick_tree.add(usize::from(a[i]), T::from(1));
        result += fenwick_tree.sum(n) - fenwick_tree.sum(usize::from(a[i]));
    }

    result
}

/// Remap the elements of a vector to the range [1, n] where n is the length of the vector.
///
/// # Arguments
///
/// * `a`: the input vector
///
/// returns: ()
/// \theta(n log n)
fn remap<T>(mut a: Vec<T>, n: usize) -> Vec<T>
where
    T: Ord
        + Clone
        + Add<Output = T>
        + Sub<Output = T>
        + From<u8>
        + Copy
        + Default
        + AddAssign
        + PartialEq
        + Debug
        + Hash,
        usize: From<T>,
{
    let mut b = a.clone();
    b.sort();
    let mut map = HashMap::new();
    let mut count = T::from(1);
    for i in 0..n {
        if i > 0 && b[i] != b[i - 1] {
            count += T::from(1);
        }
        map.insert(b[i].clone(), count);
    }
    for i in 0..n {
        a[i] = *map.get(&a[i]).unwrap();
    }
    a
}

#[test]
pub fn test_counting_inversion() {
    let mut a = vec![2, 3, 8, 6, 1];
    assert_eq!(counting_inversions(a), 5_usize);
    a = vec![1, 20, 6, 4, 5];
    assert_eq!(counting_inversions(a), 5_usize);
}
