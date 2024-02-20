
/// Returns an array containing the prefix sums of the input array.
///
/// # Arguments
///
/// * `a`: the input array
///
/// returns: Vec<T, Global>
///
/// \theta(n)
fn prefix_sums(a: Vec<i32>) -> Vec<i32>
{
    a.iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect::<Vec<_>>()
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
pub fn maximum_number_overlapping_intervals(mut intervals: Vec<(i32, i32)>) -> i32 {
    intervals.sort_by_key(|&x| x.0);
    let max = intervals.iter().map(|&x| x.1).max().unwrap() as usize;

    let mut ps = vec![0; max + 2];

    for (a, b) in intervals {
        ps[a as usize] += 1;
        ps[b as usize + 1] -= 1;
    }

    *prefix_sums(ps).iter().max().unwrap()
}

#[test]
pub fn test_maximum_number_overlapping_intervals() {
    let a = vec![(1, 3), (2, 4), (3, 6), (7, 10)];
    assert_eq!(maximum_number_overlapping_intervals(a), 3);
}
