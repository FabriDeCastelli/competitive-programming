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
    while left_capacity >= 0 {
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
