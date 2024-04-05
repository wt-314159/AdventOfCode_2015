use std::{fs, cmp::min};

const SIZE: usize = 100;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = ".#.#.# ...##. #....# ..#... #.#..# ####..";
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut lights = [[State::Off(None); SIZE]; SIZE];

    let lines: Vec<&str> = input.split_whitespace().collect();
    for y in 0..lines.len() {
        let row = lines[y];
        let mut iter = row.chars();
        for x in 0..row.chars().count() {
            let next = iter.next().unwrap();
            lights[y][x] = match next {
                '#' => State::On(None),
                '.' => State::Off(None),
                other => panic!("Can't handle '{}'", other)
            }
        }
    }

    println!("Parsed lights initial stage");

    // println!("Initial state: ");
    // print_lights(&lights);

    let step_count = 4;
    for i in 0..100 {
        iterate_lights_one_step(&mut lights);
        // println!("After {} steps: ", i + 1);
        // print_lights(&lights);
    }
    println!("Final state: ");
    print_lights(&lights);

    println!("{} lights are on", count_on_lights(&lights));
}

fn count_on_lights(lights: &[[State; SIZE]; SIZE]) -> u16 {
    let mut count: u16 = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if let State::On(_) = lights[y][x] {
                count += 1;
            }
        }
    }
    count
}

fn iterate_lights_one_step(lights: &mut [[State; SIZE]; SIZE]) {
        // get next state for all lights except edge ones
        for y in 1..SIZE - 1 {
            for x in 1..SIZE - 1 {
                light_next_state(lights, y, x);
            }
        }
        // get next state for edge lights
        for y in 0..SIZE {
            edge_light_next_state(lights, y, 0);
            edge_light_next_state(lights, y, SIZE - 1);
        }
        for x in 1..SIZE-1 {
            edge_light_next_state(lights, 0, x);
            edge_light_next_state(lights, SIZE - 1, x);
        }
    
        // update all states
        update_state(lights);
}

fn update_state(lights: &mut [[State; SIZE]; SIZE]) {
    for row in lights {
        for light in row {
            *light = match light {
                State::On(Some(true)) => State::On(None),
                State::Off(Some(true)) => State::On(None),
                State::On(Some(false)) => State::Off(None),
                State::Off(Some(false)) => State::Off(None),
                _ => panic!("Next state not set!")
            }
        }
    }
}

fn edge_light_next_state(lights: &mut [[State; SIZE]; SIZE], row: usize, col: usize) {
    let row_min = match row {
        0 => 0,
        x => x - 1
    };
    let row_max = min(row + 1, SIZE - 1);
    let col_min = match col {
        0 => 0,
        x => x - 1
    };
    let col_max = min(col + 1, SIZE - 1);

    let mut neighbours_on: u8 = 0;
    for y in row_min..=row_max {
        for x in col_min..=col_max {
            if let State::On(_) = lights[y][x] {
                neighbours_on += 1;
            }
        }
    }
    // make sure we don't count the centre light!
    if let State::On(_) = lights[row][col] {
        neighbours_on -= 1;
    }
    set_next_state(lights, row, col, neighbours_on);
}

fn light_next_state(lights: &mut [[State; SIZE]; SIZE], row: usize, col: usize) {
    let mut neighbours_on: u8 = 0;
    for y in row - 1..=row + 1 {
        for x in col - 1..=col + 1 {
            if let State::On(_) = lights[y][x] {
                neighbours_on += 1;
            }
        }
    }
    // make sure we don't count the centre light!
    if let State::On(_) = lights[row][col] {
        neighbours_on -= 1;
    }
    set_next_state(lights, row, col, neighbours_on);
}

fn set_next_state(lights: &mut [[State; SIZE]; SIZE], row: usize, col: usize, neighbours_on: u8) {
    lights[row][col] =  match lights[row][col] {
        State::On(_) => {
            let on_next = neighbours_on == 2 || neighbours_on == 3;
            State::On(Some(on_next))
        }
        State::Off(_) => {
            let on_next = neighbours_on == 3;
            State::Off(Some(on_next))
        }
    }
}

fn print_lights(lights: &[[State; SIZE]; SIZE]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = match lights[y][x] {
                State::On(_) => '#',
                State::Off(_) => '.'
            };
            print!("{}",c);
        }
        println!{};
    }
}

// store the light's next state in the enum
#[derive(Copy)]
#[derive(Clone)]
enum State {
    On(Option<bool>),
    Off(Option<bool>)
}
