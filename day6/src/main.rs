use std::fs;

const MAX: usize = 1000;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut lights = [[0; MAX]; MAX];

    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();

        if line.starts_with("turn ") {
            let start_pos = parse_pos(params[2]);
            let end_pos = parse_pos(params[4]);
            let on_or_off: bool = params[1] == "on";

            turn_on_or_off(&mut lights, on_or_off, start_pos, end_pos);
        }
        else {
            let start_pos = parse_pos(params[1]);
            let end_pos = parse_pos(params[3]);

            toggle(&mut lights, start_pos, end_pos);
        }
    }

    let mut counter = 0;
    for x in 0..MAX {
        for y in 0..MAX {
            counter += lights[x][y];
        }
    }

    println!("There are {} bulbs on", counter);
}

fn turn_on_or_off(lights: &mut [[i32; MAX]; MAX], on_or_off: bool, start_pos: (usize, usize), end_pos: (usize, usize)) {
    if end_pos.0 < start_pos.0 || end_pos.1 < start_pos.1 {
        panic!("not good!")
    }
    for x in start_pos.0..=end_pos.0 {
        for y in start_pos.1..=end_pos.1 {
            if on_or_off {
                lights[x][y] += 1;
            }
            else if lights[x][y] > 0 {
                lights[x][y] -= 1;
            }
        }
    }
}

fn toggle(lights: &mut [[i32; MAX]; MAX], start_pos: (usize, usize), end_pos: (usize, usize)) {
    if end_pos.0 < start_pos.0 || end_pos.1 < start_pos.1 {
        panic!("not good!")
    }
    for x in start_pos.0..=end_pos.0 {
        for y in start_pos.1..=end_pos.1 {
            lights[x][y] += 2;
        }
    }
}

fn parse_pos(pos_str: &str) -> (usize, usize) {
    let nums: Vec<&str> = pos_str.split(',').collect();
    let x_pos = nums[0].parse::<usize>().expect("Failed to parse to usize");
    let y_pos = nums[1].parse::<usize>().expect("Failed to parse to usize");
    return (x_pos, y_pos);
}