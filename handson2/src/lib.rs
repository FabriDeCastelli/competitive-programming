#[derive(Debug)]
pub struct SegmentTree<P, U, C>
where
    P: Fn(i32, i32) -> i32,
    U: Fn(i32, i32) -> i32,
    C: Fn(i32, i32) -> i32,
{
    length: usize,
    data: Vec<i32>,
    lazy_data: Vec<Option<i32>>,
    propagate: P,
    update: U,
    combine: C,
}

impl<P, U, C> SegmentTree<P, U, C>
where
    P: Fn(i32, i32) -> i32,
    U: Fn(i32, i32) -> i32,
    C: Fn(i32, i32) -> i32,
{
    pub fn from_vec(a: &[i32], propagate: P, update: U, combine: C) -> Self {
        let length = a.len();
        let data = vec![0; 4 * length];
        let lazy_data = vec![None; 4 * length];
        let mut segment_tree = SegmentTree {
            length,
            data,
            lazy_data,
            propagate,
            update,
            combine,
        };
        segment_tree.populate(a, 0, length - 1, 0);
        segment_tree
    }

    fn populate(&mut self, data: &[i32], min_pos: usize, max_pos: usize, current_pos: usize) {
        // Base case: leaves are the same as the data
        if min_pos == max_pos {
            self.data[current_pos] = data[max_pos];
            return;
        }

        let mid = (min_pos + max_pos) / 2;
        let left_child = left_child(current_pos);
        let right_child = right_child(current_pos);

        // Left
        self.populate(data, min_pos, mid, left_child);

        // Right
        self.populate(data, mid + 1, max_pos, right_child);

        // Combine
        self.data[current_pos] = (self.propagate)(self.data[left_child], self.data[right_child]);
    }

    pub fn print(&self) {
        self.print_recursive(0, 0, self.length - 1);
    }

    // Helper function to print the segment tree recursively
    fn print_recursive(&self, pos: usize, left: usize, right: usize) {
        println!(
            "Node: {}, Range: [{}, {}], Value: {:?}, Lazy: {:?}",
            pos, left, right, self.data[pos], self.lazy_data[pos]
        );

        if left != right {
            let mid = (left + right) / 2;
            self.print_recursive(left_child(pos), left, mid);
            self.print_recursive(right_child(pos), mid + 1, right);
        }
    }

    pub fn query_normal(&mut self, l: usize, r: usize, k: i32) -> Option<i32> {
        self.q(0, 0, self.length - 1, l, r, k)
    }

    fn q(
        &mut self,
        current: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        k: i32,
    ) -> Option<i32> {
        // No overlap
        if start > r || end < l {
            return None;
        }

        // Total overlap
        if start >= l && end <= r {
            return self.lower_bound_search(current, k);
        }

        // Partial overlap
        let mid = (start + end) / 2;
        let left = self.q(left_child(current), start, mid, l, r, k);
        let right = self.q(right_child(current), mid + 1, end, l, r, k);
        match (left, right) {
            (Some(l), Some(r)) => Some((self.propagate)(l, r)),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            _ => None,
        }
    }

    fn lower_bound_search(&mut self, current: usize, k: i32) -> Option<i32> {
        if current >= self.data.len() || self.data[current] > k {
            return None;
        }

        if self.data[current] == k {
            return Some(self.data[current]);
        }

        let left_result = self.lower_bound_search(left_child(current), k);
        let right_result = self.lower_bound_search(right_child(current), k);
        if left_result.is_some() {
            return left_result;
        }
        right_result
    }

    pub fn query(&mut self, l: usize, r: usize) -> Option<i32> {
        self.query_lazy(0, 0, self.length - 1, l - 1, r - 1)
    }

    fn query_lazy(
        &mut self,
        current: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
    ) -> Option<i32> {
        if let Some(lazy) = self.lazy_data[current] {
            self.data[current] = (self.update)(self.data[current], lazy);

            if start != end {
                self.combine_or_set_lazy(left_child(current), lazy);
                self.combine_or_set_lazy(right_child(current), lazy);
            }

            self.lazy_data[current] = None;
        }

        // No overlap
        if start > r || end < l {
            return None;
        }

        // Total overlap
        if start >= l && end <= r {
            return Some(self.data[current]);
        }

        // Partial overlap
        let mid = (start + end) / 2;
        let left = self.query_lazy(left_child(current), start, mid, l, r);
        let right = self.query_lazy(right_child(current), mid + 1, end, l, r);
        match (left, right) {
            (Some(l), Some(r)) => Some((self.propagate)(l, r)),
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            _ => None,
        }
    }

    pub fn update_range(&mut self, l: usize, r: usize, value: i32) {
        self.update_range_lazy(0, 0, self.length - 1, l - 1, r - 1, value);
    }

    fn update_range_lazy(
        &mut self,
        current: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        value: i32,
    ) {
        if let Some(lazy) = self.lazy_data[current] {
            let prev = self.data[current];
            self.data[current] = (self.update)(prev, lazy);

            if start != end {
                self.combine_or_set_lazy(left_child(current), lazy);
                self.combine_or_set_lazy(right_child(current), lazy);
            }

            self.lazy_data[current] = None;
        }

        // No overlap
        if start > end || l > end || r < start {
            return;
        }

        // Total overlap
        if l <= start && end <= r {
            // update current node
            self.data[current] = (self.update)(self.data[current], value);

            // if not a leaf, propagate to children
            if start != end {
                self.combine_or_set_lazy(left_child(current), value);
                self.combine_or_set_lazy(right_child(current), value);
            }

            return;
        }

        // Partial overlap
        let mid = (start + end) / 2;
        let left = left_child(current);
        let right = right_child(current);
        self.update_range_lazy(left, start, mid, l, r, value);
        self.update_range_lazy(right, mid + 1, end, l, r, value);
        self.data[current] = (self.propagate)(self.data[left], self.data[right]);
    }

    fn combine_or_set_lazy(&mut self, current: usize, value: i32) {
        if let Some(lazy) = self.lazy_data[current] {
            self.lazy_data[current] = Some((self.combine)(lazy, value));
        } else {
            self.lazy_data[current] = Some(value);
        }
    }
}

pub fn left_child(index: usize) -> usize {
    index * 2 + 1
}

pub fn right_child(index: usize) -> usize {
    index * 2 + 2
}

// Testing

use std::fmt::Debug;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub struct Test<T> {
    data: Vec<T>,
    queries: Vec<(usize, usize, Option<i32>)>,
    expected_outputs: Vec<i32>,
}

impl<T> Test<T> {
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_queries(&self) -> &Vec<(usize, usize, Option<i32>)> {
        &self.queries
    }

    pub fn get_expected_outputs(&self) -> &Vec<i32> {
        &self.expected_outputs
    }
}

pub fn get_tests(directory: &str, file_number: usize) -> Test<i32> {
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
    let _ = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    // Read the second line for the array
    binding = file_iter_input.next().unwrap();
    iter = binding.split_whitespace();
    let data = iter
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut queries = Vec::new();
    let mut expected_outputs = Vec::new();

    for _ in 0..m {
        binding = file_iter_input.next().unwrap();
        iter = binding.split_whitespace();

        // Update query
        if iter.next().unwrap().parse::<usize>().unwrap() == 0 {
            let l = iter.next().unwrap().parse::<usize>().unwrap();
            let r = iter.next().unwrap().parse::<usize>().unwrap();
            let k = iter.next().unwrap().parse::<i32>().unwrap();
            queries.push((l, r, Some(k)));

        // Max query
        } else {
            let output = file_iter_output.next().unwrap().parse::<i32>().unwrap();
            let l = iter.next().unwrap().parse::<usize>().unwrap();
            let r = iter.next().unwrap().parse::<usize>().unwrap();
            queries.push((l, r, None));
            expected_outputs.push(output);
        }
    }

    Test {
        data,
        queries,
        expected_outputs,
    }
}

/// ----------- Exercise 2 -----------

pub fn get_tests_ex2(directory: &str, file_number: usize) -> Test<(i32, i32)> {
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
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let mut data = Vec::with_capacity(n);

    for _ in 0..n {
        binding = file_iter_input.next().unwrap();
        iter = binding.split_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        data.push((x, y));
    }

    let mut queries = Vec::new();
    let mut expected_outputs = Vec::new();

    for _ in 0..m {
        binding = file_iter_input.next().unwrap();
        iter = binding.split_whitespace();

        let output = file_iter_output.next().unwrap().parse::<i32>().unwrap();
        let l = iter.next().unwrap().parse::<usize>().unwrap();
        let r = iter.next().unwrap().parse::<usize>().unwrap();
        let k = iter.next().unwrap().parse::<i32>().unwrap();
        queries.push((l, r, Some(k)));
        expected_outputs.push(output);
    }

    Test {
        data,
        queries,
        expected_outputs,
    }
}
