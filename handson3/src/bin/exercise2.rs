use handson3::{get_tests_ex2, design_a_course};

pub fn main() {
    let n = 11;

    for i in 0..n {

        let test = get_tests_ex2("data/exercise2", i);
        let data = test.get_data();
        let expected_output = test.get_expected_output();


        // --------- Solver 2 ---------
        let output = design_a_course(data.clone(), data.len());
        assert_eq!(output, expected_output, "Test {} failed", i)

    }
}