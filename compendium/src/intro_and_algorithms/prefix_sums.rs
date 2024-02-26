use crate::intro_and_algorithms::sweep_line::prefix_sums;
use std::usize;

/// Given an array of n booleans and a list of m queries of the form (i, j), return a vector
/// of m integers where the i-th integer is the number of pairs of consecutive elements in the
/// subarray a[i...j] that are equal.
///
/// # Arguments
///
/// * `a`: the input array
/// * `queries`: the list of queries
///
/// returns: Vec<i32, Global>
/// /theta(n + m)
pub fn ilya_and_queries(a: Vec<bool>, queries: Vec<(usize, usize)>) -> Vec<i32> {
    let m = queries.len();
    let n = a.len();
    let mut result = Vec::with_capacity(m);

    let mut ps = vec![0; n];
    for i in 0..n - 1 {
        ps[i] = i32::from(a[i] == a[i + 1]);
    }

    ps = prefix_sums(ps);

    for (i, j) in queries {
        result.push(ps[j - 1] - ps[i - 1])
    }

    result
}

/// Given an array of n integers and a list of m range_sum(i, j) queries, permute the array
/// a such that the sum of results of the queries is maximized.
///
/// # Arguments
///
/// * `a`: the input array
/// * `queries`: the list of queries
///
/// returns: i32
/// \theta(n log n + m)
/// (not \theta(n log n + m log m) because we can use counting sort on the queries)
pub fn little_girls_and_maximum(mut a: Vec<i32>, queries: Vec<(usize, usize)>) -> i32 {
    let n = a.len();
    let mut ps = vec![0; n + 1];

    for (i, j) in queries {
        ps[i - 1] += 1;
        ps[j] -= 1;
    }

    ps = prefix_sums(ps);
    ps.sort_by(|a, b| b.cmp(a));
    a.sort_by(|a, b| b.cmp(a));

    ps.iter().zip(a.iter()).fold(0, |mut res, (x, y)| {
        res = res + x * y;
        res
    })
}

#[test]
pub fn test_ilya_and_queries() {
    let mut a = vec![true, false, true, true, false];
    let queries = vec![(1, 1), (2, 4), (1, 5)];
    let mut result = ilya_and_queries(a, queries.clone());
    assert_eq!(result, vec![0, 1, 1]);

    a = vec![true, true, true, true, true];
    result = ilya_and_queries(a, queries.clone());
    assert_eq!(result, vec![0, 2, 4]);

    a = vec![false, true, false, true, false];
    result = ilya_and_queries(a, queries.clone());
    assert_eq!(result, vec![0, 0, 0]);
}

#[test]
pub fn test_little_girls_and_maximum() {
    let mut a = vec![1, 2, 3, 4, 5];
    let mut queries = vec![(1, 3), (2, 4), (1, 5)];
    assert_eq!(little_girls_and_maximum(a, queries), 38);

    a = vec![5, 2, 4, 1, 3];
    queries = vec![(1, 5), (2, 3), (2, 3)];
    assert_eq!(little_girls_and_maximum(a, queries), 33);
}
