/// Sliding Window Maximum: returns an array of length n - k + 1
/// with the maximum element in each window of size k of a
///
/// # Arguments
///
/// * `a`: the input array
/// * `k`: the window size
///
/// returns: (the array of maximums)
///
// \theta(nk)
pub fn swm_slow<T>(a: Vec<T>, k: usize) -> Vec<T>
where
    T: Ord + Default + Copy,
{
    let n = a.len();
    let mut result = Vec::with_capacity(n - k + 1);

    for i in 0..n - k + 1 {
        let mut max = a[i];
        for j in i..i + k {
            max = a[j].max(max);
        }
        result.push(max);
    }

    result
}

// TODO: implement swm_heap and swm_bst

use std::collections::VecDeque;

/// Sliding Window Maximum with Double Ended Queues
///
/// # Arguments
///
/// * `a`: the input array
/// * `k`: the window size
///
/// returns: Vec<T, Global>
///
/// \theta(n)
/// invariants:
///     - every index 0 <= i < n is inserted and removed exactly once in the deque
///     - at every iteration i, the deque is sorted in non-increasing order
pub fn swm<T>(a: Vec<T>, k: usize) -> Vec<T>
where
    T: Ord + Default + Copy,
{
    let n = a.len();
    let mut result = Vec::with_capacity(n - k + 1);
    let mut deque = VecDeque::with_capacity(n);

    for i in 0..k {
        while !deque.is_empty() && a[i] > a[*deque.back().unwrap()] {
            deque.pop_back();
        }
        deque.push_back(i);
    }
    result.push(a[*deque.back().unwrap()]);

    // in the front of the deque we always have the maximum
    for i in k..n {
        // remove maximum values that are not in the window anymore
        while !deque.is_empty() && *deque.front().unwrap() <= (i - k) {
            deque.pop_front();
        }
        // if a[i] is greater than back element, it is removed (a[i] wins)
        while !deque.is_empty() && a[i] > a[*deque.back().unwrap()] {
            deque.pop_back();
        }
        deque.push_back(i);
        // the maximum is always in the front of the queue
        result.push(a[*deque.front().unwrap()])
    }

    result
}

#[test]
pub fn test_swm() {
    let a = vec![1, 3, 5, 4, 3];
    let res_1 = swm_slow(a.clone(), 2);
    let res_2 = swm(a.clone(), 2);
    assert_eq!(res_1, vec![3, 5, 5, 4]);
    assert_eq!(res_2, vec![3, 5, 5, 4]);
}
