use core::hash::Hash;
use std::collections::HashMap;
use std::fmt::Display;

/// Returns the pointers to the longest k-good segment of the input array.
/// A subarray is called k-good if it contains no more than k different values.
/// Hashing is used to solve the problem.
/// Other variants as BST implementations are efficient as well.
///
/// # Arguments
///
/// * `a`: the input array
/// * `k`: the value k
///
/// returns: (usize, usize)
///
/// \theta(n log n) due to std::collections::HashMap
pub fn longest_k_good_segments<T>(a: Vec<T>, k: usize) -> (usize, usize)
where
    T: Ord + Copy + Default + Hash + Display
{
    let n = a.len();
    let mut i = 0;
    let mut j = 0;
    let mut best_i = 0;
    let mut best_j = 0;
    let mut best_count = 0;

    let mut support: HashMap<T, i32> = HashMap::with_capacity(n);

    while j < n {

        *support.entry(a[j]).or_insert(0) += 1;
        while support.len() > k {
            if let Some(count) = support.get_mut(&a[i]) {
                *count -= 1;
                if *count == 0 {
                    support.remove(&a[i]);
                }
            }
            i = i + 1;
        }
        if j - i + 1 > best_count {
            best_count = j - i + 1;
            best_i = i;
            best_j = j;

        }
        j = j + 1
    }

    (best_i, best_j)

}

#[test]
pub fn test_k_good() {
    let a = vec![1, 1, 2, 3, 3, 4, 4, 4, 1, 3, 2, 3];
    assert_eq!(longest_k_good_segments(a, 3), (3, 9));
}