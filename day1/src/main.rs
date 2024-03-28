use std::{fs, io};

fn main() {
    // let mut entries = fs::read_dir(".").expect("Failed to read directory")
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>().expect("Failed to collect");

    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);

    let mut floor = 0;
    let mut negative_index = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            other => panic!("Unexpected char: {}", other)
        }
        // index of first character to cause count to be -1
        if (floor == -1 && negative_index == 0) {
            negative_index = i + 1;
        }
    }

    println!("Floor number: {}", floor);
    println!("Index of negative char {}", negative_index);
}
