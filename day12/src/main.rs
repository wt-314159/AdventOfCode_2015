use std::{fs, cmp::max};
use fancy_regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //println!("{:?}", input);
    //let input = "{\"f\":5,\"d\":{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5},\"e\":{\"e\":[1,2,3,4],\"f\":5},\"h\":21}";
    println!("Input length: {}", input.len());

    // Part 1
    //println!("Sum of all numbers: {}", sum_all_numbers(&input));
    // Part 2
    println!("Sum of all non-red numbers: {}", sum_all_non_red_numbers(&input));
}

#[allow(dead_code)]
fn sum_all_numbers(input: &str) -> i32 {
    let numbers_regex = Regex::new("-?[0-9]+").unwrap();
    //let reg_matches: Vec<&str> = numbers_regex.find_iter(&input).map(|x| x.unwrap().as_str()).collect();

    let mut running_count = 0;
    for m in numbers_regex.find_iter(&input) {
        let num_str = m.unwrap().as_str();
        let num = num_str.parse::<i32>().expect(&format!("Failed to parse '{}'", num_str));
        running_count += num;
    }
    running_count
}

fn sum_all_non_red_numbers(input: &str) -> i32 {
    // Could do this with JSON but going to attempt to do it without
    let mut brackets: Vec<(usize, char)> = Vec::new();

    // Find the indices of the open and close brackets and arrays
    for (i, c) in input.chars().enumerate() {
        match c {
            '}' | '{' | '[' | ']' => brackets.push((i, c)),
            _ => continue
        }
    }

    parse_object(input, &brackets, 0).0
}

fn parse_object(input: &str, brackets: &Vec<(usize, char)>, start_index: usize) -> (i32, usize) {
    // find all relevant bits of string for current object
    let mut strings: Vec<&str> = Vec::new();
    let mut non_array_strings: Vec<&str> = Vec::new();
    let mut obj_depth = 0;
    let mut arr_depth = 0;
    let mut prev_array_close = start_index;
    let mut prev_array_open = start_index;
    let mut prev_bracket_close = start_index;
    let mut end_index = start_index;
    let bracket_vec_index = brackets.iter().position(|x| x.0 == start_index).unwrap();

    for bracket in brackets.iter().skip(bracket_vec_index + 1) {
        if bracket.1 == '{' {
            if obj_depth == 0 {
                strings.push(&input[prev_bracket_close..bracket.0]);
                // add string since previous array close to non-array-strings,
                // only if we aren't currently in an array (array open greater 
                // than array close)
                if arr_depth == 0 { 
                    let most_recent_bracket = max(prev_array_close, prev_bracket_close);
                    non_array_strings.push(&input[most_recent_bracket..bracket.0]);
                }
            }
            obj_depth += 1;
        }
        else if bracket.1 == '}' {
            if obj_depth == 0 {
                strings.push(&input[prev_bracket_close..bracket.0]);
                let most_recent_bracket = max(prev_array_close, prev_bracket_close);
                non_array_strings.push(&input[most_recent_bracket..bracket.0]);
                // end of current object
                end_index = bracket.0;
                break;
            }
            prev_bracket_close = bracket.0;
            obj_depth -= 1;
        }
        else if bracket.1 == '[' {
            if obj_depth == 0 && arr_depth == 0 {
                let most_recent_bracket = max(prev_array_close, prev_bracket_close);
                non_array_strings.push(&input[most_recent_bracket..bracket.0]);
            }
            prev_array_open = bracket.0;
            arr_depth += 1;
        }
        else if bracket.1 == ']' {
            arr_depth -= 1;
            prev_array_close = bracket.0;
        }
    }

    if (start_index == 476) {
        println!("Something's up");
    }

    let mut sum = 0;
    let contains_red = non_array_strings.iter().any(|x| x.contains("red"));
    if contains_red {
        println!("red object");
    }
    else {
        for str in strings {
            sum += sum_all_numbers(str);
        }
        println!("{} total found in object {} -> {}.", sum, start_index, end_index);

        let mut inner_end_index = start_index;
        loop {
            let bracket_inner_end_index = brackets.iter().position(|x| x.0 == inner_end_index).unwrap();
            if let Some(next_start_bracket) = brackets.iter().skip(bracket_inner_end_index + 1).find(|x| x.1 == '{') {
                if next_start_bracket.0 < end_index {
                    let result = parse_object(input, brackets, next_start_bracket.0);
                    sum += result.0;
                    inner_end_index = result.1;
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
    }
    // return the sum
    (sum, end_index)
}