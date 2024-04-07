use std::{fs, collections::HashMap, cmp::Reverse};
use priority_queue::PriorityQueue;
#[allow(unused_imports)]
use regex::{Captures, Regex, RegexSet};
use rand::{thread_rng, seq::SliceRandom};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let lines: Vec<&str> = input.split("\n").collect();
    let mut products: Vec<(Regex, &str)> = Vec::new();
    let mut replacements: HashMap<&str, Vec<String>> = HashMap::new();
    for i in 0..lines.len() - 2 {
        let line = lines[i];
        let params: Vec<&str> = line.split_whitespace().collect();
        let entry = replacements.entry(params[0]).or_insert(Vec::new());
        entry.push(String::from(params[2]));

        products.push((Regex::new(params[2]).unwrap(), params[0]));
    }
    
    let med_molecule = lines.last().unwrap();
    println!("Parsing complete");

    // Part 1
    //find_num_unique_molecules(med_molecule, replacements);

    // reverse sort by length of product
    // products.sort_by(|a, b| b.0.as_str().len().cmp(&a.0.as_str().len()));
    // println!("Products sorted");
    
    let num_steps = a_star_decompose(med_molecule, &mut products);
    println!("Found solution requiring {} steps", num_steps);
}

#[allow(dead_code)]
fn find_num_unique_molecules(med_molecule: &str, replacements: HashMap<&str, Vec<String>>) {
    let mut molecules: Vec<String> = Vec::new();

    for replcmnt in replacements {
        all_molecules_from_replacement(med_molecule, replcmnt, &mut molecules);
    }

    println!("{} unique new molecules found!", molecules.len()); 
}

// credit to https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4cu5b
// Doesn't necessarily find the shortest number of steps however
#[allow(dead_code)]
fn try_decompose_shuffle(med_molecule: &str, products: &mut Vec<(Regex, &str)>) -> u32 {
    let mut target = String::from(med_molecule);
    let mut steps = 0;
    let mut dead_ends = 0;

    while target != "e" {
        let tmp = String::from(&target);
        for rep in &mut *products {
            if rep.0.is_match(&target) {
                target = rep.0.replace(&target, rep.1).to_string();
                steps += 1;
            }
        }

        if tmp == target {
            dead_ends += 1;
            target = String::from(med_molecule);
            steps = 0;
            products.shuffle(&mut thread_rng())
        }
    }
    println!("{} dead ends explored", dead_ends);
    steps
}

// Pretty basic / crude A* implementation, estimated cost function is very basic, but 
// it's tricky to think of a good way to estimate the cost of a molecule when we're 
// trying to decompose it that wouldn't take so long to run as to be counter-productive.
// Seems like the key with this entire puzzle problem is to do depth first searches, but
// when you run into a dead_end or don't have a match, start again from the beginning and
// take a different path, rather than going back to the previous point on the dead end
// path and looking for a different way
fn a_star_decompose(med_molecule: &str, products: &mut Vec<(Regex, &str)>) -> usize {
    let start_node = Node { molecule: String::from(med_molecule), num_steps: 0 };
    let total_cost = start_node.estimate_cost();

    let mut priority_queue = PriorityQueue::new();
    priority_queue.push(start_node, Reverse(total_cost));

    loop {
        // pop the 'best' node from the queue
        let node = priority_queue.pop().unwrap();
        // If the node is "e", return num steps
        if node.0.molecule == "e" {
            return node.0.num_steps;
        }
        // go through every match, replace and and new node
        for prod in &mut *products {
            if prod.0.is_match(&node.0.molecule) {
                let new = prod.0.replace(&node.0.molecule, prod.1).to_string();
                let new_node = Node { molecule: new, num_steps: node.0.num_steps + 1 };
                let est_cost = new_node.estimate_cost();
                priority_queue.push(new_node, Reverse(est_cost));
            }
        }
        // Could maybe make this more efficient by tracking the parent node for each node,
        // and if no matches were found for a given node, updating the estimated cost of it's
        // parent node to deprioritise it slightly, so if a node leads to several dead ends we 
        // start looking at other nodes
    }
}


fn all_molecules_from_replacement(med_molecule: &str, mol_replacements: (&str, Vec<String>), molecules: &mut Vec<String>) {
    let regex = Regex::new(mol_replacements.0).unwrap();

    for cap in regex.find_iter(med_molecule) {
        for replacement in &mol_replacements.1 {
            let range = cap.range();
            let mut new_molecule = String::from(med_molecule);
            new_molecule.replace_range(range, &replacement);

            if !molecules.contains(&new_molecule) {
                molecules.push(new_molecule);
            }
        }
    }
}

#[derive(Hash)]
#[derive(PartialEq, Eq)]
struct Node {
    molecule: String,
    num_steps: usize,
}

impl Node {
    fn estimate_cost(&self) -> usize {
        self.num_steps + self.estimate_distance()
    }

    fn estimate_distance(&self) -> usize {
        // very basic heuristic, assume each transformation reduces 
        // length by about 3, so number of transformations still 
        // required is difference between molecule length and target
        // length (which is 1) divided by 3
        (self.molecule.len() - 1) / 3
    }
}
