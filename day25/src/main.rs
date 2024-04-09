#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max, time::Instant};
// use fancy_regex::Regex;
// use regex::Regex;

const START_CODE: usize = 20151125;
const MULTIPLIER: usize = 252533;
const MODULO: usize = 33554393;

fn main() {
    // Need the value at row 2981, column 3075
    // Should be at index 18331559
    // Means we would have to do the find_next_Value operation that many times
    let test_index = find_index_of_row_and_column(4, 3);
    println!("Number in row 4 column 3 should be: {}", test_index + 1);

    let code_index = find_index_of_row_and_column(2981, 3075);
    println!("Code should be at index {} (would contain number {} in diagram)", code_index, code_index + 1);

    // time finding the first 1000 numbers 
    // 1000 numbers took 18.8 microseconds
    let start = Instant::now();
    let mut curr_code = START_CODE;
    for i in 1..1000 {
        curr_code = find_next_value(curr_code);
    }
    let elapsed = start.elapsed();
    println!("Found 1000th code ({}) in {:.2?}", curr_code, elapsed);
}

fn find_next_value(prev_value: usize) -> usize {
    let big_number = prev_value * MULTIPLIER;
    big_number % MODULO
}

fn find_index_of_row_and_column(row: usize, col: usize) -> usize {
    let mut r = 1;
    let mut c = 1;
    let mut index = 0;

    while r != row || c != col {
        // move diagonally up and right
        if r > 1 {
            r -=1;
            c += 1;
        }
        else {
            r = c + 1;
            c = 1;
        }
        index += 1;
    }
    index
}