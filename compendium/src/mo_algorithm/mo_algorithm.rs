use num_traits::real::Real;

pub fn mo_algorithm(
    a: Vec<usize>,
    queries: Vec<(usize, usize)>,
    add: Box<dyn Fn(usize, &mut Vec<i32>, &mut i32) -> ()>,
    remove: Box<dyn Fn(usize, &mut Vec<i32>, &mut i32) -> ()>,
) -> Vec<i32> {
    let n = a.len();
    let q = queries.len();

    let sqrt = (n as f32).sqrt() as usize + 1;
    let mut permutation: Vec<_> = (0..q).collect();
    let mut sorted_queries = queries.clone();
    sorted_queries.sort_by_key(|&(l, r)| (l / sqrt, r));
    permutation.sort_by_key(|&i| (queries[i].0 / sqrt, queries[i].1));

    let mut result = Vec::new();
    let mut support = vec![0; n];

    let mut answer = 0;
    let mut cur_l = 0;
    let mut cur_r = 0;

    for &(l, r) in sorted_queries.iter() {

        while cur_r <= r {
            add(a[cur_r], &mut support, &mut answer);
            cur_r = cur_r + 1;
        }

        while cur_l > l {
            cur_l = cur_l - 1;
            add(a[cur_l], &mut support, &mut answer);
        }

        while cur_l < l {
            remove(a[cur_l], &mut support, &mut answer);
            cur_l = cur_l + 1;
        }
        while cur_r > r + 1 {
            cur_r = cur_r - 1;
            remove(a[cur_r], &mut support, &mut answer);
        }
        result.push(answer);
    }

    let mut permuted_answers = vec![0; result.len()];
    for (i, answer) in permutation.into_iter().zip(result) {
        permuted_answers[i] = answer;
    }

    permuted_answers

}

/// Three or more

pub fn three_or_more(a: Vec<usize>, queries: Vec<(usize, usize)>) -> Vec<i32> {


    mo_algorithm(a, queries, Box::new(|i, support, answer| {
        support[i] += 1;
        if support[i] == 3 {
            *answer += 1;
        }
    }), Box::new(|i, support, answer| {
        support[i] -= 1;
        if support[i] == 2 {
            *answer -= 1;
        }
    }))

}

pub fn power(a: Vec<usize>, queries: Vec<(usize, usize)>) -> Vec<i32> {

    mo_algorithm(a, queries, Box::new(|i, support, answer| {
        support[i] = support[i].max(0) + 1;
        *answer +=  i as i32 * (support[i].pow(2));
    }), Box::new(|i, support, answer| {
        *answer -=  i as i32 * support[i].pow(2);
        support[i] = (support[i] - 1).max(0);
    }))
}

#[test]
fn test_three_or_more() {
    let a = vec![1, 2, 3, 2, 1, 4, 2, 3, 4, 1];
    let queries = vec![(0, 9), (0, 0), (0, 3), (0, 5), (0, 6), (1, 6), (2, 6), (0, 7), (0, 8)];
    let result = three_or_more(a.clone(), queries);
    assert_eq!(result, vec![2, 0, 0, 0, 1, 1, 0, 1, 1]);
}

#[test]
fn test_power() {
    let a = vec![1, 2, 3, 2, 1, 4, 2, 3, 4, 1];


    let queries = vec![(0, 2), (7, 7), (4, 8)];
    let result = power(a.clone(), queries);
    assert_eq!(result, vec![6, 3, 26]);


    let queries = vec![(0, 2), (4, 8)];
    let result = power(a.clone(), queries);
    assert_eq!(result, vec![6, 26]);
}