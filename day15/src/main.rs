use std::{fs, cmp::max, str::FromStr, collections::HashMap};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut ingredients: Vec<Ingredient> = Vec::new();
    for line in input.split("\n") {
        ingredients.push(line.parse::<Ingredient>().unwrap());
    }

    println!("Ingredients parsed");
    let score = test_recipes(&ingredients);
    println!("Highest scoring recipe: {}", score);
}

#[allow(dead_code)]
fn test_test_input(ingredients: &Vec<Ingredient>) -> i32 {
    let total_teaspoons = 100;
    let mut highest_score = 0;

    for i in 0..=total_teaspoons {
        let j = total_teaspoons - i;

        let mut recipe: Vec<(&Ingredient, i32)> = Vec::new();
        recipe.push((&ingredients[0], i));
        recipe.push((&ingredients[1], j));

        let score = score_recipe(&recipe);
        if score > highest_score {
            highest_score = score;
        }
    }
    highest_score
}

fn test_recipes(ingredients: &Vec<Ingredient>) -> i32 {
    let total_teaspoons = 100;
    let mut highest_score = 0;

    for i in 0..=total_teaspoons {
        let remaining_teaspoons = total_teaspoons - i;
        for j in 0..=remaining_teaspoons {
            let remaining_teaspoons = remaining_teaspoons - j;
            for k in 0..=remaining_teaspoons {
                let h = remaining_teaspoons - k;
                
                let mut recipe: Vec<(&Ingredient, i32)> = Vec::new();
                recipe.push((&ingredients[0], i));
                recipe.push((&ingredients[1], j));
                recipe.push((&ingredients[2], k));
                recipe.push((&ingredients[3], h));

                let score = score_recipe(&recipe);
                if score > highest_score {
                    highest_score = score;
                    println!("{} tsps {}, {} tsps {}, {} tsps {}, {} tsps {}", i, ingredients[0].name, j, ingredients[1].name, k, ingredients[2].name, h, ingredients[3].name);
                }
            }
        }
    }
    highest_score
}

fn score_recipe(recipe: &Vec<(&Ingredient, i32)>) -> i32 {
    // Part 2, comment out the calories block for Part 1
    let total_calories: i32 = recipe.iter().map(|(i, x)| i.calories * x).sum();
    if total_calories != 500 {
        return 0;
    } 
    // ^ Comment out up to here ^
    let total_capacity: i32 = recipe.iter().map(|(i, x)| i.capacity * x).sum();
    if total_capacity < 0 {
        return 0;
    }
    let total_durability: i32 = recipe.iter().map(|(i, x)| i.durability * x).sum();
    if total_durability < 0 {
        return 0;
    }
    let total_flavour: i32 = recipe.iter().map(|(i, x)| i.flavour * x).sum();
    if total_flavour < 0 { 
        return 0; 
    }
    let total_texture: i32 = recipe.iter().map(|(i, x)| i.texture * x).sum();
    if total_texture < 0 {
        return 0;
    } 
    total_capacity * total_durability * total_flavour * total_texture
}

struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32
}

#[derive(Debug)]
struct IngredientErr(String);

impl FromStr for Ingredient {
    type Err = IngredientErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let params: Vec<&str> = s.split_whitespace().collect();
        let name = String::from(params[0].trim_matches(':'));
        let capacity = params[2].trim_matches(',').parse::<i32>().unwrap();
        let durability = params[4].trim_matches(',').parse::<i32>().unwrap();
        let flavour = params[6].trim_matches(',').parse::<i32>().unwrap();
        let texture = params[8].trim_matches(',').parse::<i32>().unwrap();
        let calories = params[10].parse::<i32>().unwrap();

        Ok(Ingredient { name, capacity, durability, flavour, texture, calories })
    }
}
