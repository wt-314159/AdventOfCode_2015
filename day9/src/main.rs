use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut locations: Vec<&str> = Vec::new();
    let mut distances: HashMap<(&str, &str), u16> = HashMap::new();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let dist = params[4].parse::<u16>().expect("Failed to parse");
        // Really lazy, but just inserting into distances HashpMap in both orders
        // (e.g. as distance between London and Berlin as well as distances between 
        // Berlin and London, just so we can search either without having to worry
        // about order)
        distances.insert((params[0], params[2]), dist);
        distances.insert((params[2], params[0]), dist);
        insert_if_not_present(&mut locations, params[0]);
        insert_if_not_present(&mut locations, params[2]);
    }

    println!("Distances entered");

    // Really ugly way to do this, using a recursive function would look nicer
    // and be more maintainable, however would require having multiple mutable 
    //  references to min_dist and take a bit of thinking about, so for now do
    // it the hacky way
    let available:[usize; 8] = [0,1,2,3,4,5,6,7];
    let mut min_dist = 65_535;
    let mut max_dist = 0;
    for i0 in 0..locations.len() {
        let index = i0;
        let first_slice = &available[..index];
        let second_slice = &available[index+1..];
        let prev_place = locations[available[i0]];
        let available = [first_slice, second_slice].concat();
        
        let dist = 0;
        for i1 in 0..locations.len()-1 {
            let index = i1;
            let first_slice = &available[..index];
            let second_slice = &available[index+1..];

            let current_place = locations[available[index]];
            let available = [first_slice, second_slice].concat();
            let temp_dist = distances[&(prev_place, current_place)];
            let prev_place = current_place;
            let dist = dist + temp_dist;

            for i2 in 0..locations.len()-2 {
                let index = i2;
                let first_slice = &available[..index];
                let second_slice = &available[index+1..];

                let current_place = locations[available[index]];
                let available = [first_slice, second_slice].concat();
                let temp_dist = distances[&(prev_place, current_place)];
                let prev_place = current_place;
                let dist = dist + temp_dist;

                for i3 in 0..locations.len()-3 {
                    let index = i3;
                    let first_slice = &available[..index];
                    let second_slice = &available[index+1..];

                    let current_place = locations[available[index]];
                    let available = [first_slice, second_slice].concat();
                    let temp_dist = distances[&(prev_place, current_place)];
                    let prev_place = current_place;
                    let dist = dist + temp_dist;

                    for i4 in 0..locations.len()-4 {
                        let index = i4;
                        let first_slice = &available[..index];
                        let second_slice = &available[index+1..];

                        let current_place = locations[available[index]];
                        let available = [first_slice, second_slice].concat();
                        let temp_dist = distances[&(prev_place, current_place)];
                        let prev_place = current_place;
                        let dist = dist + temp_dist;

                        for i5 in 0..locations.len()-5 {
                            let index = i5;
                            let first_slice = &available[..index];
                            let second_slice = &available[index+1..];

                            let current_place = locations[available[index]];
                            let available = [first_slice, second_slice].concat();
                            let temp_dist = distances[&(prev_place, current_place)];
                            let prev_place = current_place;
                            let dist = dist + temp_dist;

                            for i6 in 0..locations.len()-6 {
                                let index = i6;
                                let first_slice = &available[..index];
                                let second_slice = &available[index+1..];

                                let current_place = locations[available[index]];
                                let available = [first_slice, second_slice].concat();
                                let temp_dist = distances[&(prev_place, current_place)];
                                let prev_place = current_place;
                                let dist = dist + temp_dist;

                                // Only one option left
                                let current_place = locations[available[0]];
                                let temp_dist = distances[&(prev_place, current_place)];
                                let dist = dist + temp_dist;

                                if dist < min_dist {
                                    min_dist = dist;
                                }
                                if dist > max_dist {
                                    max_dist = dist;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Minimum distance: {}", min_dist);
    println!("Maximum distance: {}", max_dist);
}

fn insert_if_not_present<'a>(locations: &mut Vec<&'a str>, place: &'a str) {
    if !locations.contains(&place) {
        locations.push(place);
    }
}

// fn recursively_find_shortest_route(
//     locations: &Vec<&str>,
//     distances: &HashMap<(&str, &str), u16>,
//     current_dist: u16,
//     available: &[u16],
//     depth: usize,
//     prev_index: usize) {

//     if (depth == 8) {
        
//     }
//     for i in 0..8-depth {

//     }
// }