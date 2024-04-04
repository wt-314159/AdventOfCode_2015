use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    // test with 20, 15, 10, 5, and 5
    // let input = "20 15 10 5 5";
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let params = input.split_whitespace();
    let mut containers: Vec<u16> = params.map(|x| x.parse::<u16>().unwrap()).collect();

    let total_volume: u16 = 150;
    let mut containers_used: HashMap<usize, u16> = HashMap::new();
    let num_options = find_viable_groupings(0, &mut containers, total_volume, 0, &mut containers_used);
    println!("Number of options: {}", num_options);
    
    let min_used = containers_used.iter().map(|x| x.0).min().unwrap();
    let num_ways = containers_used[min_used];
    println!("{} ways of using minimum number of containers ({})", num_ways, min_used);
}

fn find_viable_groupings(
    mut viable_options: u16, 
    remaining_containers: &mut Vec<u16>, 
    remaining_volume: u16, 
    start_index: usize,
    containers_used: &mut HashMap<usize, u16>) -> u16 {
    for i in start_index..remaining_containers.len() {
        if remaining_containers[i] > remaining_volume {
            continue;
        }
        let vol = remaining_containers[i];
        let remaining_volume = remaining_volume - vol;

        if remaining_volume == 0 {
            viable_options += 1;
            let num_used = 20 - remaining_containers.len() + 1;
            let entry = containers_used.entry(num_used).or_insert(0);
            *entry += 1;
        }
        else {
            let removed = remaining_containers.remove(i);
            viable_options = find_viable_groupings(viable_options, remaining_containers, remaining_volume, i, containers_used);
            remaining_containers.insert(i, removed);
        }
    }
    viable_options
}
