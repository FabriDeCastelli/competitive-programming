use handson3::{get_tests_ex1, holiday_planning};

pub fn main() {
    let n = 5;

    for i in 0..n {
        let test = get_tests_ex1("data/exercise1", i);
        let data = test.get_data();
        let expected_output = test.get_expected_output();

        // --------- Solver 2 ---------

        let n = data.len();
        let d = data[0].len();
        let output = holiday_planning(data, n, d);
        assert_eq!(output, expected_output, "Test {} failed", i);

    }
}