use handson2::{get_tests_ex2, SegmentTree};
use std::cmp::min;

fn main() {
    let n = 7;
    for i in 0..n {
        let test = get_tests_ex2("data/exercise2", i);

        assert_eq!(
            test.get_queries().len(),
            test.get_expected_outputs().len(),
            "Error in reading test data"
        );

        let data = test.get_data();

        // Count array
        let mut count = vec![0; data.len() + 1];
        for &elem in data {
            count[elem.0 as usize] += 1;
            count[elem.1 as usize + 1] -= 1;
        }

        // Prefix sums array
        let mut prefix_sum: Vec<i32> = vec![0; data.len() + 1];
        prefix_sum[0] = count[0];
        for i in 1..data.len() {
            prefix_sum[i] = prefix_sum[i - 1] + count[i];
        }

        // Last element is useless for the segment tree
        prefix_sum.pop();

        // Actually we only need propagate because we are not updating any values
        let mut segment_tree = SegmentTree::from_vec(&prefix_sum, min, min, min);
        // segment_tree.print();

        let queries = test.get_queries();
        let expected_outputs = test.get_expected_outputs();

        let mut results: Vec<i32> = Vec::new();
        for query in queries {
            if let Some(v) = query.2 {
                results.push(
                    if segment_tree.query_normal(query.0, query.1, v).is_some() {
                        1
                    } else {
                        0
                    },
                )
            };
        }

        assert!(
            results
                .iter()
                .zip(expected_outputs.iter())
                .all(|(a, b)| a == b),
            "Exercise 2: test failed!"
        );
    }
}
