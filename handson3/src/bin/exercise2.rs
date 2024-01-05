use handson3::{get_tests_ex2, design_a_course};

pub fn main() {
    let n = 10;

    for i in 0..n {
        let test = get_tests_ex2("data/exercise1", i);
        let data = test.get_data();
        let expected_output = test.get_expected_output();

        // --------- Solver 2 ---------

    }
}