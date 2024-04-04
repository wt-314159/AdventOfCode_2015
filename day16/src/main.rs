use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut sues: Vec<Sue> = Vec::new();
    for line in input.split("\n") {
        sues.push(line.parse::<Sue>().unwrap());
    }

    let test_sue = Sue {
        number: 1000,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };


    for sue in sues {
        if sue.is_match(&test_sue) {
            println!("Matching sue number is {}", sue.number);
            println!("Matching sue traits: {:?}", sue);
        }
    }
}

#[derive(Debug)]
struct Sue {
    number: u16,
    children: Option<u16>,
    cats: Option<u16>,
    samoyeds: Option<u16>,
    pomeranians: Option<u16>,
    akitas: Option<u16>,
    vizslas: Option<u16>,
    goldfish: Option<u16>,
    trees: Option<u16>,
    cars: Option<u16>,
    perfumes: Option<u16>,
}

#[derive(Debug)]
struct ParseErr(String);

impl Sue {
    fn default(number: u16) -> Sue {
        Sue {
            number,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    fn is_match(&self, test_sue: &Sue) -> bool {
        // for Part 1, replace all functions on left with is_match
        is_match(test_sue.children.unwrap(), self.children) &&
        is_greater(test_sue.cats.unwrap(), self.cats) &&
        is_match(test_sue.samoyeds.unwrap(), self.samoyeds) &&
        is_less(test_sue.pomeranians.unwrap(), self.pomeranians) &&
        is_match(test_sue.akitas.unwrap(), self.akitas) &&
        is_match(test_sue.vizslas.unwrap(), self.vizslas) &&
        is_less(test_sue.goldfish.unwrap(), self.goldfish) &&
        is_greater(test_sue.trees.unwrap(), self.trees) &&
        is_match(test_sue.cars.unwrap(), self.cars) &&
        is_match(test_sue.perfumes.unwrap(), self.perfumes)
    }
}

fn is_match(expected: u16, actual: Option<u16>) -> bool {
    if let Some(number) = actual {
        return number == expected;
    }
    true
}

fn is_greater(expected: u16, actual: Option<u16>) -> bool {
    if let Some(number) = actual {
        return number > expected;
    }
    true
}

fn is_less(expected: u16, actual: Option<u16>) -> bool {
    if let Some(number) = actual {
        return number < expected;
    }
    true
}

impl FromStr for Sue {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let params: Vec<&str> = s.split_whitespace().collect();
        let number = params[1].trim_matches(':').parse::<u16>().unwrap();
        let mut sue = Sue::default(number);

        for i in 2..params.len() {
            match params[i] {
                "children:" => {
                    sue.children = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap())
                }
                "cats:" => sue.cats = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap()),
                "samoyeds:" => {
                    sue.samoyeds = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap())
                }
                "pomeranians:" => {
                    sue.pomeranians = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap())
                }
                "akitas:" => {
                    sue.akitas = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap())
                }
                "vizslas:" => {
                    sue.vizslas = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap())
                }
                "goldfish:" => {
                    sue.goldfish = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap())
                }
                "trees:" => {
                    sue.trees = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap())
                }
                "cars:" => sue.cars = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap()),
                "perfumes:" => {
                    sue.perfumes = Some(params[i + 1].trim_matches(',').parse::<u16>().unwrap())
                }
                _ => continue,
            }
        }
        Ok(sue)
    }
}
