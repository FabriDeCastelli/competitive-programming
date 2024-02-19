/// Runs binary search with the given predicate.
///
/// # Arguments
///
/// * `a`: the input array
/// * `v`: the value to be searched
/// * `l`: the starting index of the binary search
/// * `r`: the ending index of the binary search
/// * `pred`: the predicate function to compare items
///
/// returns: (T, usize)
///
/// \theta(\log n)

pub fn binary_search<T, F>(a: Vec<T>, mut l: usize, mut r: usize, pred: F) -> Option<usize>
where
    T: Ord + Copy,
    F: Fn(T) -> bool,
{
    assert!(r <= a.len(), "Binary Search: end out of range");

    let mut ans = None;

    while l < r {
        let middle = l + (r - l) / 2;

        if pred(a[middle]) {
            l = middle + 1;
            ans = Some(middle);
        } else {
            r = middle;
        }
    }

    ans
}

/// Given an integer, returns it square root rounded down to the nearest integer.
///
/// # Arguments
///
/// * `v`: the integer
///
/// returns: sqrt(v)
///
/// \theta(\log v)
pub fn sqrt(v: i32) -> usize {
    let a: Vec<i32> = (0..v).collect();
    binary_search(a.clone(), 0, a.len(), |x| x * x <= v).unwrap()
}

/// Solves the social distancing problem: given an array of pairs of integers andn integer c, find
/// the minimum distance d such that exactly c integers are placed within the intervals with their
/// minimum distance to be >= d.
/// https://usaco.org/index.php?page=viewproblem2&cpid=1038
///
/// # Arguments
///
/// * `intervals`: the array of intervals
/// * `c`: the number of integers to be placed
///
/// returns: (the minimum distance)
///
/// \theta (n \log l)
pub fn social_distancing(mut intervals: Vec<(i32, i32)>, c: i32) -> usize {
    assert!(intervals.iter().all(|&(a, b)| a <= b));

    let l = intervals.iter().fold(0, |mut sum, (a, b)| {
        sum = sum + b - a + 1;
        sum
    });

    intervals.sort();

    let predicate = |d| {
        let mut total = 1;

        let mut current = intervals[0].0;
        for &interval in intervals.iter() {
            while interval.0.max(current + d) <= interval.1 {
                current = interval.0.max(current + d);
                total = total + 1;
            }
        }

        total >= c
    };

    binary_search((1..=l + 1).collect(), 0, l as usize, predicate).unwrap()
}

#[test]
pub fn test_sqrt() {
    let v = 4;
    assert_eq!(sqrt(v), 2);
}

#[test]
pub fn test_social_distancing() {
    let mut a = vec![(0, 3), (5, 7)];
    let mut c = 4;
    assert_eq!(social_distancing(a, c), 1);

    a = vec![(0, 3), (5, 7), (10, 13)];
    c = 3;

    assert_eq!(social_distancing(a, c), 5);
}
