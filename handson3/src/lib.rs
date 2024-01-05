pub fn holiday_planning(data: &Vec<Vec<i32>>, n: usize, d: usize) -> i32 {

    // This array contains the cumulative weight of each attraction in each city
    // The first element of the array is 0, meaning that we can decide not to include it in the trip
    let weights: Vec<_> = vec![(0..=d).collect::<Vec<_>>(); n];

    // This array contains the cumulative value (reward) of each attraction in each city
    // Again here, the first element of the array is 0, meaning that we can decide not to include it in the trip
    let values: Vec<_> = data
        .iter()
        .map(|attractions| {
            let mut result = vec![0; d+1];
            attractions
                .iter()
                .enumerate()
                .for_each(|(i, attraction)| {
                    result[i + 1] = result[i] + attraction;
                });
            result
        })
        .collect();

    // Dynamic programming table
    let mut table = vec![vec![0; d + 1]; n + 1];

    // For every city
    for i in 1..=n {

        // For every attraction in the city, including the possibility of not visiting it
        for j in 0..=d {

            let current_weight = weights[i - 1][j];
            let current_value = values[i - 1][j];

            // Recursion formula, choose the best value among all candidates in the same row
            // Only if it fits (start from current_weight)
            for k in current_weight..=d {
                table[i][k] = max(table[i][k], table[i - 1][k - current_weight] + current_value);
            }
        }

    }

    // The result is the bottom right element of the table
    table[n][d]

}


pub fn design_a_course(data: Vec<(i32, i32)>, n: usize) -> i32 {

    let mut table = vec![1; n];

    // Sort topics based on the first element of the couple (beauty in this case)
    let mut sorted_topics = data.clone();
    sorted_topics.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));

    // Start from second element because the LIS = 1 counting the first element only
    for i in 1..n {
        for j in 0..i {
            // Contribution only if difficulty is greater than the other element's difficulty
            if sorted_topics[i].1 > sorted_topics[j].1 {
                table[i] = table[i].max(table[j] + 1);
            }
        }
    }

    // The result is the maximum of the dynamic programming table
    table.into_iter().max().unwrap_or(0)
}



// -------- Testing --------

use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Test<T> {
    data: Vec<T>,
    expected_output: i32,
}

impl<T> Test<T> {
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_expected_output(&self) -> i32 {
        self.expected_output
    }
}


/// ----------- Exercise 1 -----------

pub fn get_tests_ex1(directory: &str, file_number: usize) -> Test<Vec<i32>> {
    let input_file_path = format!("{}/input{}.txt", directory, file_number);
    let output_file_path = format!("{}/output{}.txt", directory, file_number);

    let mut file_iter_input = BufReader::new(File::open(input_file_path).unwrap())
        .lines()
        .map(|x| x.unwrap());

    let mut file_iter_output = BufReader::new(File::open(output_file_path).unwrap())
        .lines()
        .map(|x| x.unwrap());

    // Read the first line for n and m
    let mut binding = file_iter_input.next().unwrap();
    let mut iter = binding.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let d = iter.next().unwrap().parse::<usize>().unwrap();


    let mut data = Vec::with_capacity(n);
    let mut attraction = Vec::with_capacity(d);

    for _ in 0..n {
        binding = file_iter_input.next().unwrap();
        iter = binding.split_whitespace();
        for _ in 0..d {
            let num_activities = iter.next().unwrap().parse::<i32>().unwrap();
            attraction.push(num_activities);
        }
        data.push(attraction.clone());
        attraction.clear();
    }

    binding = file_iter_output.next().unwrap();
    iter = binding.split_whitespace();
    let expected_output = iter.next().unwrap().parse::<i32>().unwrap();


    Test {
        data,
        expected_output
    }
}

/// ----------- Exercise 1 -----------

pub fn get_tests_ex2(directory: &str, file_number: usize) -> Test<(i32, i32)> {
    let input_file_path = format!("{}/input{}.txt", directory, file_number);
    let output_file_path = format!("{}/output{}.txt", directory, file_number);

    let mut file_iter_input = BufReader::new(File::open(input_file_path).unwrap())
        .lines()
        .map(|x| x.unwrap());

    let mut file_iter_output = BufReader::new(File::open(output_file_path).unwrap())
        .lines()
        .map(|x| x.unwrap());

    // Read the first line for n
    let mut binding = file_iter_input.next().unwrap();
    let mut iter = binding.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let mut data = Vec::with_capacity(n);

    for _ in 0..n {
        binding = file_iter_input.next().unwrap();
        iter = binding.split_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        data.push((x, y));

    }

    binding = file_iter_output.next().unwrap();
    iter = binding.split_whitespace();
    let expected_output = iter.next().unwrap().parse::<i32>().unwrap();

    Test {
        data,
        expected_output,
    }
}