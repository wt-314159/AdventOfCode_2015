use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut even = true;
    let mut current_pos = (0, 0);
    let mut robot_pos = (0, 0);
    visited.insert(current_pos, 2);

    for c in input.chars() {
        // bit messy, trying to be quick rather than write nice looking code
        match even {
            true => update_pos(&mut current_pos, c, &mut visited),
            false => update_pos(&mut robot_pos, c, &mut visited)
        };
        even = !even;
    }

    println!("{} houses visited", visited.len());
}

fn update_pos(position: &mut (i32, i32), c: char, visited: &mut HashMap<(i32, i32), i32>) {
    match c {
        '^' => position.1 += 1,
        '>' => position.0 += 1,
        'v' => position.1 -= 1,
        '<' => position.0 -= 1,
        _ => panic!("Unrecognised char! {}", c)
    }
    let visit = visited.entry(*position).or_insert(0);
    *visit += 1;
}
