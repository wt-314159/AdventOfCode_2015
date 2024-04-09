#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut weights = Vec::new();
    #[allow(unused_variables)]
    for line in input.split("\n") {
        let w = line.parse::<usize>().unwrap();
        weights.push((w, false));
    }

    let total_weight: usize = weights.iter().map(|x| x.0).sum();
    // Part 2, for Part 1, divide by 3
    let group_weight = total_weight / 4;

    weights.reverse();

    println!("Total weight {}, group weight: {}", total_weight, group_weight);

    let group1 = fill_group(&mut weights, group_weight);
    println!("Finished!")
}

fn fill_group(weights: &mut Vec<(usize, bool)>, target_weight: usize) {
    let mut curr_weight = 0;
    let mut group: Vec<usize> = Vec::new();
    let mut qe_s: Vec<usize> = Vec::new();

    let group_weight = find_next_weight(weights, target_weight, &mut group, curr_weight, 100, 0, &mut qe_s);
    qe_s.sort();
    let lowest = qe_s.first().unwrap();
    println!("Lowest QE: {}", lowest);
}

fn print_weights(weights: &Vec<usize>) {
    print!(" {}", weights.first().unwrap());
    for w in weights.iter().skip(1) {
        print!(" {}", w);
    }
    print!("\n");
}

fn find_next_weight(
    weights: &mut Vec<(usize,bool)>, 
    target: usize, 
    group: &mut Vec<usize>, 
    mut curr_weight: usize, 
    mut min_weights: usize, 
    start: usize, 
    qe_s: &mut Vec<usize>) -> usize {

    let remaining_weight = target - curr_weight;
    if curr_weight == target {
        return min_weights;
    }
    for i in start..weights.len() {
        let weight = &weights[i];
        if weight.1 || weight.0 > remaining_weight {
            continue;
        }
        group.push(weight.0);
        curr_weight += weight.0;
        weights[i].1 = true;

        if curr_weight == target {
            if group.len() <= min_weights {
                println!("Solution found using {} weights", group.len());
                print_weights(group);
                let qe = get_quantum_entanglement(group);
                println!("QE = {}", qe);
                if qe_s.contains(&qe) {
                    println!("Duplicate");
                }
                qe_s.push(qe);
                min_weights = group.len();
            }
        }
        else {
            min_weights = find_next_weight(weights, target, group, curr_weight, min_weights, i + 1, qe_s);
        }
        // undo our choice and try again
        group.pop();
        curr_weight -= weights[i].0;
        weights[i].1 = false;
    }
    min_weights
}

fn get_quantum_entanglement(weights: &Vec<usize>) -> usize {
    let mut qe = 1;
    for w in weights {
        qe *= w;
    }
    qe
}
