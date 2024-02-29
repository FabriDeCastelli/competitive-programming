
/// Computes the nth fibonacci number.
///
/// # Arguments
///
/// * `n`: the index of the fibonacci number to find
///
/// returns: usize
/// \theta(n)
fn fibonacci(n: usize) -> usize {
    if n <= 1 {
        return n;
    }

    let mut n2 = n - 2;
    let mut n1 = n - 1;
    let mut res = 0;
    for _ in 2..=n {
        res = n1 + n2;
        n2 = n1;
        n1 = res;
    }

    res
}


/// Rod cutting problem.
///
/// # Arguments
///
/// * `prices`: the prices of the rod
/// * `n`: the length of the rod
///
/// returns: usize
/// \theta(n^2)
pub fn rod_cutting(prices: Vec<usize>, n: usize) -> usize {
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        let mut max_val = 0;
        for j in 0..i {
            max_val = max_val.max(prices[j] + dp[i - j - 1]);
        }
        dp[i] = max_val;
    }
    dp[n]
}


/// Given an n by m matrix, find the minimum cost path from the top left element
/// to the bottom right element.
///
/// # Arguments
///
/// * `a`: the input matrix
///
/// returns: i32
/// \theta(m * n)
pub fn minimum_cost_path(a: Vec<Vec<i32>>) -> i32 {
    let n = a.len();
    assert_ne!(n, 0);
    let m = a[0].len();

    let mut t = vec![vec![i32::MAX; m + 1]; n + 1];
    t[0][1] = 0;
    t[1][0] = 0;

    for i in 1..=n {
        for j in 1..=m {
            t[i][j] = a[i - 1][j - 1] + t[i - 1][j].min(t[i][j - 1]);
        }
    }
    t[n][m]
}

/// Counts the number of ways to construct a string of 1s and 0s such that there
/// are no consecutive zeros.
///
/// # Arguments
///
/// * `n`: the length of the string
///
/// returns: usize
/// \theta(n)
pub fn zero_11_ss(n: usize) -> usize {

    if n <= 1 {
        return n + 1;
    }

    let mut n2 = n - 2;
    let mut n1 = n - 1;
    let mut res = 0;
    for _ in 2..=n {
        res = n1 + n2;
        n2 = n1;
        n1 = res;
    }

    res
}


#[test]
pub fn test_fibonacci() {
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(7), 13);
    assert_eq!(fibonacci(15), 610);
}

#[test]
pub fn test_rod_cutting() {
    let prices = vec![1, 5, 8, 9, 10, 17, 17, 20];
    assert_eq!(rod_cutting(prices, 8), 22);
}

#[test]
pub fn test_minimum_cost_path() {
    let a = vec![
        vec![1, 3, 1],
        vec![1, 5, 1],
        vec![4, 2, 1]
    ];
    assert_eq!(minimum_cost_path(a), 7);
}

#[test]
pub fn test_zero_11_ss() {
    assert_eq!(zero_11_ss(3), 5);
}
