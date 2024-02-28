use crate::fenwick_trees::fenwick_tree::FenwickTree;

/// Given a set of n segments, for each segments count the number of smaller segments that it
/// contains. The right endpoint of each segment is unique. We assume that each segment is such that
/// (l, r) subset of [1, 2n] and l < r.
/// https://codeforces.com/problemset/problem/652/D?locale=en
///
/// # Arguments
///
/// * `segments`: the input vector of segments
///
/// returns: Vec<i32, Global>
///
/// \theta( n log n)
pub fn nested_segments(mut segments: Vec<(i32, i32)>) -> Vec<i32> {
    let n = segments.len();
    let mut result = Vec::new();

    segments.sort_by(|a, b| a.0.cmp(&b.0));

    let mut fenwick_tree: FenwickTree<i32> = FenwickTree::with_capacity(n);

    for &(_, r) in &segments {
        fenwick_tree.add(r as usize, 1);
    }

    for &(_, r) in &segments {
        result.push(fenwick_tree.sum(r as usize - 1));
        fenwick_tree.add(r as usize, -1);
    }

    result
}

#[test]
pub fn test_nested_segments() {
    let segments = vec![(1, 4), (1, 3), (2, 2)];
    let result = nested_segments(segments);
    assert_eq!(result, vec![2, 1, 0]);
}
