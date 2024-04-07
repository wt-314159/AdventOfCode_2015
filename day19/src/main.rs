use std::{fs, collections::HashMap};
use regex::{Captures, Regex, RegexSet};

const U32_MAX: u32 = 4_294_967_295u32;

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
    
    let transforms: Vec<(Regex, &Vec<String>)> = replacements.iter().map(|(x, v)| (Regex::new(x).unwrap(), v)).collect();
    let med_molecule = lines.last().unwrap();
    println!("Parsing complete");

    // Part 1
    //find_num_unique_molecules(med_molecule, replacements);

    // reverse sort by length of product
    products.sort_by(|a, b| b.0.as_str().len().cmp(&a.0.as_str().len()));
    println!("Products sorted");

    // Go through the products from longest to shortest and find every match, replacing iteratively,
    // until hopefully we've decomposed it down to just 'e'
    //let num_steps = decompose_molecule(med_molecule, &products, 0);

    let regex_set = &RegexSet::new(products.iter().map(|(x, _)| x.as_str())).unwrap();
    //let regex_set = RegexSet::new(transforms.iter().map(|x| x.0.as_str())).unwrap();

    let num_steps = try_decompose_all_ways(med_molecule, &products, regex_set, 0, U32_MAX);

    //let mut molecules: Vec<String> = vec![String::from("e")];
    //let num_steps = get_all_molecules_from(&med_molecule, 0, &regex_set, &transforms, &mut molecules, 0).0;
    println!("Have reduced molecule in {} steps!", num_steps);
}

#[allow(dead_code)]
fn find_num_unique_molecules(med_molecule: &str, replacements: HashMap<&str, Vec<String>>) {
    let mut molecules: Vec<String> = Vec::new();

    for replcmnt in replacements {
        all_molecules_from_replacement(med_molecule, replcmnt, &mut molecules);
    }

    println!("{} unique new molecules found!", molecules.len()); 
}

fn get_all_molecules_from(
    med_molecule: &str, 
    mol_index: usize, 
    regex_set: &RegexSet,
    replacements: &Vec<(Regex, &Vec<String>)>, 
    mut molecules: &mut Vec<String>, 
    mut num_steps: u32) -> (u32, bool) {

    let existing_mols_len = molecules.len();
    let curr_mol = String::from(&molecules[mol_index]);
    all_molecules_from_regex_set(&curr_mol, regex_set, replacements, &mut molecules);

    num_steps += 1;
    let mut found = false;
    println!("{} unique molecules found in {} steps", molecules.len(), num_steps);

    for i in existing_mols_len..molecules.len() {
        let mol = &molecules[i];

        if mol.len() >= med_molecule.len() {
            if mol == med_molecule {
                println!("We did it! Took {} steps", num_steps);
                return (num_steps, true);
            }
            molecules.remove(i);
            println!("Not a match");
        }

        (num_steps, found) = get_all_molecules_from(med_molecule, i, regex_set, replacements, molecules, num_steps);
    }
    (num_steps, found)
}

fn try_decompose_all_ways(med_molecule: &str, products: &Vec<(Regex, &str)>, regex_set: &RegexSet, num_steps: u32, mut min_steps: u32) -> u32 {
    let hits = regex_set.matches(&med_molecule);
    for hit in hits {
        let product = &products[hit];

        for mat in product.0.find_iter(&med_molecule) {
            let mut reduced_molecule = String::from(med_molecule);
            reduced_molecule.replace_range(mat.range(), product.1);
            let num_steps = num_steps + 1;
            //print!(".");
            if reduced_molecule == "e" {
                println!("Molecule fully reduced in {} steps", num_steps);
                if num_steps < min_steps {
                    return num_steps;
                }
                return min_steps;
            }
            min_steps = try_decompose_all_ways(&reduced_molecule, products, regex_set, num_steps, min_steps);
        }
    }
    min_steps
}

fn decompose_molecule(med_molecule: &str, products: &Vec<(Regex, &str)>, mut num_steps: u16) -> u16 {
    let mut reduced_molecule = String::from(med_molecule);

    while reduced_molecule != "e" {
        // Find the longest product that has a match in the molecule
        let longest_match = products.iter()
            .find(|(x, _)| x.is_match(&reduced_molecule))
            .expect(&format!("Dead end! Molecule currently: '{}'", reduced_molecule));
        reduced_molecule = longest_match.0.replace_all(&reduced_molecule, |_: &Captures| {
            num_steps += 1;
            longest_match.1
        }).to_string();
    }
    num_steps
}

fn decompose_molecule_old(med_molecule: &str, products: &Vec<(Regex, &str)>, mut num_steps: u16) -> u16 {
    let mut reduced_molecule = String::from(med_molecule);
    for prod in products {
        let reg_string = prod.0.as_str();
        if reduced_molecule == "e" {
            return num_steps;
        }

        // Replace all occurrences of match, then check for more matches
        // of same pattern that might now have emerged
        if prod.0.is_match(&reduced_molecule) {
            reduced_molecule = prod.0.replace_all(&reduced_molecule, |_: &Captures| {
                num_steps += 1;
                prod.1
            }).to_string();
            // Go one level deeper and start again from beginning of products vec,
            // so we should always get the longest match
            break;
        }
    }
    num_steps = decompose_molecule_old(&reduced_molecule, products, num_steps);
    num_steps
}

fn all_molecules_from_regex_set(molecule: &str, regex_set: &RegexSet, replacements: &Vec<(Regex, &Vec<String>)>, molecules: &mut Vec<String>) {
    let hits = regex_set.matches(&molecule);
    for hit in hits {
        let product = &replacements[hit];

        for mat in product.0.find_iter(&molecule) {
            for replacement in product.1 {
                let range = mat.range();
                let mut new_molecule = String::from(molecule);
                new_molecule.replace_range(range, &replacement);

                if !molecules.contains(&new_molecule) {
                    molecules.push(new_molecule);
                }
            }
        }
    }
}

fn all_molecules_from_regex(molecule: &str, mol_regex: &(Regex, &Vec<String>), molecules: &mut Vec<String>) {
    // testing if match should be cheaper than find_iter
    if !mol_regex.0.is_match(molecule) {
        return;
    }
    for hit in mol_regex.0.find_iter(molecule) {
        for replacement in mol_regex.1 {
            let range = hit.range();
            let mut new_molecule = String::from(molecule);
            new_molecule.replace_range(range, &replacement);

            if !molecules.contains(&new_molecule) {
                molecules.push(new_molecule);
            }
        }
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
