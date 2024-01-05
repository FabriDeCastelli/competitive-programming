use handson2::{get_tests, SegmentTree};
use std::cmp::{max, min};

fn main() {
    let n = 10;
    for i in 0..n {
        let test = get_tests("data/exercise1", i);
        let data = test.get_data();
        let mut segment_tree = SegmentTree::from_vec(data, max, min, min);
        let expected_outputs = test.get_expected_outputs();
        let queries = test.get_queries();

        let mut results: Vec<i32> = Vec::new();
        for query in queries {
            match query.2 {
                Some(v) => segment_tree.update_range(query.0, query.1, v),
                None => results.push(segment_tree.query(query.0, query.1).unwrap()),
            };
        }

        assert!(
            results
                .iter()
                .zip(expected_outputs.iter())
                .all(|(a, b)| a == b),
            "Exercise 1: test failed!"
        );
    }
}
