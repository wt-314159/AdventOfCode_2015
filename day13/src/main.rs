use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut happy_map: HashMap<(&str, &str), i32> = HashMap::new();
    let mut guest_list: Vec<&str> = Vec::new();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let mut happiness = params[3].parse::<i32>().unwrap();
        if params[2] == "lose" {
            happiness = -happiness;
        }

        let person1 = params.first().unwrap();
        let person2 = params.last().unwrap().trim_end_matches('.');
        happy_map.insert((person1, person2), happiness);
        if !guest_list.contains(person1) {
            guest_list.push(person1)
        }
    }

    // Part 2, adding myself to guest list with happiness 0 for everyone
    let my_name = "Will";
    for guest in &guest_list {
        happy_map.insert((guest, my_name), 0);
        happy_map.insert((my_name, guest), 0);
    }
    guest_list.push(my_name);

    println!("happy table constructed");
    let mut seating_arrangement: Vec<usize> = Vec::new();
    // Part 2, for Part 1, remove last element '8'
    let mut remaining_guests: Vec<usize> = vec![0,1,2,3,4,5,6,7,8];
    let happiest_arrangement = find_best_arrangment(&happy_map, &guest_list, &mut remaining_guests, &mut seating_arrangement, 0);

    println!("Happiest layout found, happiness: {}", happiest_arrangement);
}

fn find_best_arrangment(
    happy_map: &HashMap<(&str, &str), i32>, 
    guest_list: &Vec<&str>, 
    remaining_guests: &mut Vec<usize>, 
    seating_arrangement: &mut Vec<usize>,
    mut happiest: i32) -> i32 {
    
    for i in 0..remaining_guests.len() {
        let guest = remaining_guests[i];
        // recursively call function
        if remaining_guests.len() != 1 {
            let mut remaining_guests = remaining_guests.clone();
            remaining_guests.remove(i);
            let mut seating_arrangement = seating_arrangement.clone();
            seating_arrangement.push(guest);
            let sub_happiest = find_best_arrangment(happy_map, guest_list, &mut remaining_guests,&mut seating_arrangement, happiest);
            if sub_happiest > happiest {
                happiest = sub_happiest;
            }
        }
        else {
            seating_arrangement.push(guest);
            let total_happiness = count_happiness(happy_map, guest_list, seating_arrangement);
            if total_happiness > happiest {
                happiest = total_happiness;
                print_seating_arrangement(guest_list, seating_arrangement, total_happiness);
            }
        }
    }
    happiest
}

fn count_happiness(
    happy_map: &HashMap<(&str, &str), i32>, 
    guest_list: &Vec<&str>, 
    seating_arrangement: &Vec<usize>) -> i32 {

    let mut total_happiness = 0;
    let mut seated_guests: Vec<&str> = Vec::new();

    for i in seating_arrangement {
        let guest = guest_list[*i];

        // will skip for the first guest
        if let Some(prev_guest) = seated_guests.last() {
            total_happiness += two_guest_happiness(happy_map, prev_guest, guest);
        }
        seated_guests.push(guest);
    }
    // just have to add the happiness for the last and first guests
    total_happiness += two_guest_happiness(happy_map, seated_guests.first().unwrap(), seated_guests.last().unwrap());
    total_happiness
}

fn two_guest_happiness(happy_map: &HashMap<(&str, &str), i32>, guest1: &str, guest2: &str) -> i32 {
    let guest1_happiness = happy_map[&(guest1, guest2)];
    let guest2_happiness = happy_map[&(guest2, guest1)];
    guest1_happiness + guest2_happiness
}

fn print_seating_arrangement(guest_list: &Vec<&str>, seating_arrangement: &Vec<usize>, happiness: i32) {
    // Example: 
    // Alice -> Bob -> Carol -> David -> Eric -> Frank -> George -> Mallory: 734
    for i in 0..seating_arrangement.len() {
        let index = seating_arrangement[i];
        let guest = guest_list[index];
        
        if i < seating_arrangement.len() - 1 {
            print!("{} -> ", guest);
        }
        else {
            println!("{}: {}", guest, happiness);
        }
    }
}
