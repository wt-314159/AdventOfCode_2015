use std::{fs, collections::HashMap, hash::Hash, str::FromStr};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut all_reindeer: HashMap<String, Reindeer> = HashMap::new();
    for line in input.split("\n") {
        let reindeer = line.parse::<Reindeer>().unwrap();
        let name = String::from(reindeer.name.clone());
        all_reindeer.insert( name, reindeer);
    }
    
    println!("Parsed reindeers");
    let race_length: u16 = 2503;
    // Part 1
    //run_timed_race(&all_reindeer, race_length);

    // Part 2
    let mut furthest_distance = 0;
    let mut reindeer_points: HashMap<String, u16> = HashMap::new();
    for reindeer in &all_reindeer {
        reindeer_points.insert(String::from(reindeer.0), 0);
    }

    for sec in 0..race_length {
        // Move each reindeer forward by one second
        for reindeer in all_reindeer.iter_mut() {
            let distance = reindeer.1.step_one_sec();
            if distance > furthest_distance {
                furthest_distance = distance;
            }
        }
        // find all the reindeer that have travelled the furthest so far
        let furthest = all_reindeer.iter().filter(|x| x.1.distance == furthest_distance);
        // award them each one point
        for leader in furthest {
            *reindeer_points.get_mut(leader.0).unwrap() += 1;
        }
    }

    println!("Points after race: ");
    for reindeer in &reindeer_points {
        println!("{} got {} points", reindeer.0, reindeer.1);
    }
    let winner = reindeer_points.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    println!("winner is {}, who got {} points", winner.0, winner.1);
}

fn run_timed_race(all_reindeer: &HashMap<String, (Reindeer, u16)>, race_length: u16) {
    let mut furthest = 0;
    
    for reindeer_points in all_reindeer {
        let reindeer = &reindeer_points.1.0;
        let distance = reindeer.distance_flown(race_length);
        println!("{} flew {} km", reindeer.name, distance);
        if distance > furthest {
            furthest = distance;
            println!("{} is in the lead", reindeer.name);
        }
    }

    println!("The distance winner flew {}", furthest);
}

#[derive(Hash)]
struct Reindeer {
    name: String,
    speed: u16,
    flight_time: u16,
    rest_time:  u16,
    flying: bool,
    time_until_switch: u16,
    distance: u16
}

#[derive(Debug)]
struct ReindeerErr {
    msg: String
}

impl FromStr for Reindeer{
    type Err = ReindeerErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let params: Vec<&str> = s.split_whitespace().collect();
        let speed = params[3].parse::<u16>().unwrap();
        let flight_time = params[6].parse::<u16>().unwrap();
        let rest_time = params[13].parse::<u16>().unwrap();

        Ok(Reindeer { name: String::from(params[0]), speed, flight_time, rest_time, flying: true, time_until_switch: flight_time, distance: 0 })
    }
}

impl Eq for Reindeer {
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Reindeer {
    fn distance_flown(&self, time: u16) -> u16 {
        let flight_and_rest_time = self.flight_time + self.rest_time;
        let flight_distance = self.flight_time * self.speed;

        let mut total_time: u16 = 0;
        let mut total_distance: u16 = 0;

        // Note, could achieve the same thing with modulo and division
        while total_time + flight_and_rest_time < time {
            total_distance += flight_distance;
            total_time += flight_and_rest_time;
        }
        let time_remaining = time - total_time;
        if time_remaining >= self.flight_time {
            total_distance += flight_distance;
        }
        else {
            total_distance += time_remaining * self.speed;
        }

        total_distance
    }

    fn step_one_sec(&mut self) -> u16 {
        self.time_until_switch -= 1;
        if self.flying {
            self.distance += self.speed;
            if self.time_until_switch == 0 {
                self.flying = false;
                self.time_until_switch = self.rest_time;
            }
        }
        else if self.time_until_switch == 0 {
            self.flying = true;
            self.time_until_switch = self.flight_time;
        }
        self.distance
    }
}
