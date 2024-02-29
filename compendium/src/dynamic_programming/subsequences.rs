/// Computes the Longest Common Subsequence of two strings.
///
/// # Arguments
///
/// * `s1`: the first string
/// * `s2`: the second string
///
/// returns: usize
/// \theta(n * m)
pub fn longest_common_subsequence(s1: &str, s2: &str) -> usize {
    let n = s1.len();
    let m = s2.len();
    let mut t = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                t[i][j] = t[i - 1][j - 1] + 1;
            } else {
                t[i][j] = t[i - 1][j].max(t[i][j - 1]);
            }
        }
    }
    t[n][m]
}

/// Returns the length of the longest increasing subsequence in a vector.
///
/// # Arguments
///
/// * `s`: the vector
///
/// returns: usize
/// \theta(n^2)
pub fn lis(s: Vec<i32>) -> Vec<usize> {
    let n = s.len();

    let mut t = vec![1; n];
    for i in 1..n {
        let mut max = usize::MIN;
        for j in 0..i {
            if s[j] < s[i] {
                max = max.max(t[j]);
            }
        }
        t[i] = 1 + max;
    }

    t
}

pub fn longest_increasing_subsequence(s: Vec<i32>) -> usize {
    let lis = lis(s);
    *lis.iter().max().unwrap()
}

/// Returns the length of the longest decreasing subsequence in a vector.
///
/// # Arguments
///
/// * `s`: the vector
///
/// returns: usize
/// \theta(n^2)
pub fn longest_decreasing_subsequence(mut s: Vec<i32>) -> usize {
    let reversed = s.iter().copied().rev().collect();
    longest_increasing_subsequence(reversed)
}

pub fn longest_bitonic_subsequence(s: Vec<i32>) -> usize {
    let longest_incr_subsequence = lis(s.clone());
    let longest_decr_subsequence = lis(s.iter().copied().rev().collect());

    let n = s.len();

    let mut max = 0;
    for i in 0..n {
        max = max.max(longest_incr_subsequence[i] + longest_decr_subsequence[n - i - 1] - 1);
    }
    max
}

#[test]
pub fn test_longest_common_subsequence() {
    let s1 = "abcde";
    let s2 = "ace";
    assert_eq!(longest_common_subsequence(s1, s2), 3);
    let s1 = "abc";
    let s2 = "abc";
    assert_eq!(longest_common_subsequence(s1, s2), 3);
    let s1 = "abc";
    let s2 = "def";
    assert_eq!(longest_common_subsequence(s1, s2), 0);
}

#[test]
pub fn test_longest_increasing_subsequence() {
    let s = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(longest_increasing_subsequence(s), 4);
    let s = vec![0, 1, 0, 3, 2, 3];
    assert_eq!(longest_increasing_subsequence(s), 4);
    let s = vec![7, 7, 7, 7, 7, 7, 7];
    assert_eq!(longest_increasing_subsequence(s), 1);
}

#[test]
pub fn test_longest_decreasing_subsequence() {
    let s = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(longest_decreasing_subsequence(s), 4);
    let s = vec![0, 1, 0, 3, 2, 3];
    assert_eq!(longest_decreasing_subsequence(s), 4);
    let s = vec![7, 7, 7, 7, 7, 7, 7];
    assert_eq!(longest_decreasing_subsequence(s), 1);
}

#[test]
pub fn test_longest_bitonic_subsequence() {
    let s = vec![1, 11, 2, 10, 4, 5, 2, 1];
    assert_eq!(longest_bitonic_subsequence(s), 6);
    let s = vec![12, 11, 40, 5, 3, 1];
    assert_eq!(longest_bitonic_subsequence(s), 5);
    let s = vec![80, 60, 30, 40, 20, 10];
    assert_eq!(longest_bitonic_subsequence(s), 5);
}
