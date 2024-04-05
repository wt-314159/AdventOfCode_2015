use std::{fs, collections::HashMap};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = ".#.#.# ...##. #....# ..#... #.#..# ####..";
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let lines: Vec<&str> = input.split("\n").collect();
    let mut replacements: HashMap<&str, Vec<String>> = HashMap::new();
    for i in 0..lines.len() - 2 {
        let line = lines[i];
        let params: Vec<&str> = line.split_whitespace().collect();
        let entry = replacements.entry(params[0]).or_insert(Vec::new());
        entry.push(String::from(params[2]));
    }
    
    let med_molecule = lines.last().unwrap();
    println!("Parsing complete");

    for replacement in replacements {

    }
}

fn all_molecules_from_replacement(med_molecue: &str, mol_replacements: (&str, Vec<String>), molecules: &mut Vec<&str>) {
    
}
