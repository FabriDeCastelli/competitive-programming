use std::ops::Add;

/// Returns an array containing the prefix sums of the input array.
///
/// # Arguments
///
/// * `a`: the input array
///
/// returns: Vec<T, Global>
///
/// \theta(n)
pub fn prefix_sums<T>(a: Vec<T>) -> Vec<T>
where
    T: Ord + Clone + Default + Add<Output = T>,
{
    let n = a.len();
    let mut prefix_sums = Vec::with_capacity(n);
    prefix_sums.push(T::default());

    for i in 0..n - 1 {
        prefix_sums.push(prefix_sums[i].clone() + a[i].clone());
    }

    prefix_sums
}

/// Given an array of intervals of the form [a, b] with a <= b, return the maximum number
/// of intervals that overlap.
///
/// # Arguments
///
/// * `intervals`: the array intervals
///
/// returns: i32
///
/// \theta(n log n)
pub fn maximum_number_overlapping_intervals(intervals: Vec<(i32, i32)>) -> i32 {
    *overlapping_intervals(intervals).iter().max().unwrap()
}

pub fn overlapping_intervals(mut intervals: Vec<(i32, i32)>) -> Vec<i32> {
    intervals.sort_by_key(|&x| x.0);
    let max = intervals.iter().map(|&x| x.1).max().unwrap() as usize;

    let mut ps = vec![0; max + 2];

    for (a, b) in intervals {
        ps[a as usize] += 1;
        ps[b as usize + 1] -= 1;
    }

    prefix_sums(ps)
}

#[test]
pub fn test_maximum_number_overlapping_intervals() {
    let a = vec![(1, 3), (2, 4), (3, 6), (7, 10)];
    assert_eq!(maximum_number_overlapping_intervals(a), 3);
}
