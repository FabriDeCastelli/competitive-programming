pub fn knapsack(v: Vec<i32>, w: Vec<usize>, capacity: usize) -> i32 {
    let n = v.len();
    assert_eq!(n, w.len());

    let mut k = vec![vec![0; capacity + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=capacity {
            // Object i fits having capacity j
            if j >= w[i - 1] {
                k[i][j] = (k[i - 1][j - w[i - 1]] + v[i - 1]).max(k[i - 1][j]);
            } else {
                k[i][j] = k[i - 1][j];
            }
        }
    }

    k[n][capacity]
}

pub fn fractional_knapsack(v: Vec<i32>, w: Vec<usize>, capacity: usize) -> f32 {
    let n = v.len();
    assert_eq!(n, w.len());

    let mut objects = (0..n).collect::<Vec<_>>();
    objects.sort_by_key(|&i| v[i] / w[i] as i32);

    let mut res = 0.;
    let mut left_capacity = capacity;
    let mut i = 0;
    while i < n {
        if w[i] <= left_capacity {
            res = res + v[i] as f32;
        } else {
            res = res + (left_capacity / w[i]) as f32 * v[i] as f32;
        }
        i = i + 1;
        left_capacity = left_capacity - w[i];
    }

    res
}

pub fn subset_sum(s: Vec<i32>, v: usize) -> bool {
    let n = s.len();
    let mut t = vec![vec![false; v + 1]; n + 1];

    for i in 0..=n {
        t[i][0] = true;
    }

    for i in 1..=n {
        for j in 1..=v {
            if s[i - 1] <= j as i32 {
                t[i][j] = t[i - 1][j - s[i - 1] as usize] || t[i - 1][j];
            } else {
                t[i][j] = t[i - 1][j];
            }
        }
    }

    t[n][v]
}

pub fn coin_change(c: Vec<usize>, k: usize) -> usize {
    let n = c.len();
    let mut t = vec![vec![0; k + 1]; n + 1];

    for i in 0..=n {
        t[i][0] = 1;
    }

    for i in 1..=n {
        for j in 1..=k {
            if c[i - 1] <= j {
                t[i][j] = t[i - 1][j] + t[i][j - c[i - 1]];
            } else {
                t[i][j] = t[i - 1][j];
            }
        }
    }

    t[n][k]
}

#[test]
pub fn test_knapsack() {
    let v = vec![60, 100, 120];
    let w = vec![10, 20, 30];
    let capacity = 50;
    assert_eq!(knapsack(v, w, capacity), 220);
}

#[test]
pub fn test_fractional_knapsack() {
    let v = vec![60, 100, 120];
    let w = vec![10, 20, 30];
    let capacity = 50;
    assert_eq!(fractional_knapsack(v, w, capacity), 240.);
}

#[test]
pub fn test_subset_sum() {
    let s = vec![3, 34, 4, 12, 5, 2];
    assert_eq!(subset_sum(s.clone(), 9), true);
    assert_eq!(subset_sum(s.clone(), 30), false);
    assert_eq!(subset_sum(s.clone(), 14), true);
}

#[test]
pub fn test_coin_change() {
    let c = vec![1, 2, 3];
    assert_eq!(coin_change(c, 4), 4);
    let c = vec![2, 5, 3, 6];
    assert_eq!(coin_change(c, 10), 5);
}
