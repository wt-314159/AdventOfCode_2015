#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut weapons: Vec<Item> = Vec::new();
    let mut armor: Vec<Item> = Vec::new();
    let mut rings: Vec<Item> = Vec::new();

    #[allow(unused_variables)]
    for category in input.split("\n\n") {
        let lines: Vec<&str> = category.split("\n").collect();
        let first_params: Vec<&str> = lines.first().unwrap().split_whitespace().collect();
        match first_params[0] {
            "Weapons:" => weapons = create_weapons(lines),
            "Armor:" => armor = create_armor(lines),
            "Rings:" => rings = create_rings(lines),
            other => panic!("Cannot parse {}", other)
        }
    }

    // add empty options for no armor and no ring
    armor.push(Item::new_armor("no_armor", 0, 0));
    rings.push(Item::new_ring("no_ring", 0, 0 ,0));

    // Boss Stats:
    // Hit Points: 100
    // Damage: 8
    // Armor: 2
    let boss = Entity::new(8, 2);

    // have to have exactly 1 weapon
    let weapon_options = weapons.len();
    // have to have 0 or 1 armor
    let armor_options = armor.len();
    // can have no ring (1 option), 1 ring (6 options),
    // or 2 rings (5 + 4 + 3 + 2 + 1 options)
    let mut ring_options = rings.len();
    for i in 0..rings.len() - 1 {
        for j in i+1..rings.len() - 1 {
            ring_options += 1;
        }

    }

    let total_options = weapon_options * armor_options * ring_options;
    println!("Total equipment options: {}", total_options);

    let minimum_cost = find_minimum_win_cost(&weapons, &armor, &rings, &boss);
    println!("All options explored, lowest win cost was {}", minimum_cost);

    let maximum_cost = find_maximum_lose_cost(&weapons, &armor, &rings, &boss);
    println!("All options explored, highest lose cost was {}", maximum_cost);
}

fn find_minimum_win_cost(weapons: &Vec<Item>, armor: &Vec<Item>, rings: &Vec<Item>, boss: &Entity) -> i32 {
    let is_better = |curr_cost: i32, best_cost: i32| curr_cost < best_cost;
    let fight_outcome = |me: &Entity, boss: &Entity| fight(me, boss);
    find_best_cost(weapons, armor, rings, boss, 10000, true, is_better, fight_outcome)
}

fn find_maximum_lose_cost(weapons: &Vec<Item>, armor: &Vec<Item>, rings: &Vec<Item>, boss: &Entity) -> i32 {
    let is_better = |curr_cost: i32, best_cost: i32| curr_cost > best_cost;
    let fight_outcome = |me: &Entity, boss: &Entity| !fight(me, boss);
    find_best_cost(weapons, armor, rings, boss, 0, false, is_better, fight_outcome)
}

fn find_best_cost<F, G>(
    weapons: &Vec<Item>, 
    armor: &Vec<Item>, 
    rings: &Vec<Item>, 
    boss: &Entity, 
    start_cost: i32,
    can_skip: bool,
    is_better: F,
    fight_outcome: G) -> i32 where 
    F: Fn(i32, i32) -> bool, 
    G: Fn(&Entity, &Entity) -> bool {

    let mut best_cost = start_cost;
    let mut me = Entity::new(0,0);
    let mut total_cost = 0;

    // 1 weapon
    for i in 0..weapons.len() {
        let weapon = &weapons[i];

        // No point trying any further options if we've already
        // exceeded the minimum cost, doesn't matter if we win or lose
        if can_skip && !is_better(total_cost + weapon.cost, best_cost) {
            continue;
        }

        me.equip_item(weapon);
        total_cost += weapon.cost;
        
        // 0 or 1 armor 
        for j in 0..armor.len() {
            let armor = &armor[j];

            // No point trying any further options if we've already
            // exceeded the minimum cost, doesn't matter if we win or lose
            if can_skip && !is_better(total_cost + armor.cost, best_cost) {
                continue;
            }

            me.equip_item(armor);
            total_cost += armor.cost;

            // 0 or 1 ring
            for k in 0..rings.len() {
                let ring = &rings[k];

                // No point trying any further options if we've already
                // exceeded the best cost, doesn't matter if we win or lose
                if can_skip && !is_better(total_cost + ring.cost, best_cost) {
                    continue;
                }

                me.equip_item(ring);
                total_cost += ring.cost;

                me.recalculate_stats();
                if is_better(total_cost, best_cost) && fight_outcome(&me, boss)  {
                    best_cost = total_cost;
                    println!("Found better option costing: {}", best_cost);
                }

                total_cost -= ring.cost;
                me.remove_item(ring);
            }

            // last ring option is no_ring, ignore for 2 rings
            for k in 0..rings.len() - 1 {
                for m in k + 1..rings.len() - 1 {
                    let ring1 = &rings[k];
                    let ring2 = &rings[m];

                    // No point trying any further options if we've already
                    // exceeded the minimum cost, doesn't matter if we win or lose
                    if can_skip && !is_better(total_cost + ring1.cost + ring2.cost, best_cost) {
                        continue;
                    }

                    me.equip_item(ring1);
                    me.equip_item(ring2);
                    total_cost += ring1.cost;
                    total_cost += ring2.cost;

                    me.recalculate_stats();
                    if is_better(total_cost, best_cost) && fight_outcome(&me, boss) {
                        best_cost = total_cost;
                        println!("Found better option costing: {}", best_cost);
                    }

                    total_cost -= ring1.cost;
                    total_cost -= ring2.cost;
                    me.remove_item(ring1);
                    me.remove_item(ring2);
                }
            }

            total_cost -= armor.cost;
            me.remove_item(armor);
        }

        total_cost -= weapon.cost;
        me.remove_item(weapon);
    }

    me.clear_items();
    total_cost = 0;


    best_cost
}

fn fight(me: &Entity, boss: &Entity) -> bool {
    let mut damage_done = me.damage - boss.armor;
    if damage_done < 1 {
        damage_done = 1;
    }
    let mut damage_taken = boss.damage - me.armor;
    if damage_taken < 1 {
        damage_taken = 1;
    }

    // Will always win if damage done is greater or equal to damage taken
    if damage_done >= damage_taken {
        return true;
    }
    let mut my_health = me.hit_points;
    let mut boss_health = boss.hit_points;
    loop {
        boss_health -= damage_done;
        my_health -= damage_taken;
        if boss_health <= 0 {
            return true;
        }
        if my_health <= 0 {
            return false;
        }
    }
}

fn create_weapons(lines: Vec<&str>) -> Vec<Item> {
    let mut weapons: Vec<Item> = Vec::new();
    for line in lines.iter().skip(1) {
        let params: Vec<&str> = line.split_whitespace().collect();
        weapons.push(Item::new_weapon(params[0], params[1].parse().unwrap(), params[2].parse().unwrap()));
    }
    weapons
}

fn create_armor(lines: Vec<&str>) -> Vec<Item> {
    let mut armor: Vec<Item> = Vec::new();
    for line in lines.iter().skip(1) {
        let params: Vec<&str> = line.split_whitespace().collect();
        armor.push(Item::new_armor(params[0], params[1].parse().unwrap(), params[3].parse().unwrap()));
    }
    armor
}

fn create_rings(lines: Vec<&str>) -> Vec<Item> {
    let mut rings: Vec<Item> = Vec::new();
    for line in lines.iter().skip(1) {
        let params: Vec<&str> = line.split_whitespace().collect();
        let ring_name = String::from(params[0]) + params[1];
        rings.push(Item::new_ring(&ring_name, params[2].parse().unwrap(), params[3].parse().unwrap(), params[4].parse().unwrap()));
    }
    rings
}

struct Entity<'a> {
    hit_points: i32,
    damage: i32,
    armor: i32,
    items: Vec<&'a Item>
}

impl<'a> Entity<'a> {
    fn new(damage: i32, armor: i32) -> Entity<'static> {
        Entity { damage, armor, hit_points: 100, items: Vec::new() }
    }

    fn equip_item(&mut self, item: &'a Item) {
        // removed checks, we'll just be careful to equip the right numbers of each item
        match item.item_type {
            ItemType::Weapon => {
                // if self.items.iter().any(|x| x.item_type == ItemType::Weapon) {
                //     panic!("Cannot dual wield!");
                // }
                self.items.push(item);
            }
            ItemType::Armor => {
                // if self.items.iter().any(|x| x.item_type == ItemType::Armor) {
                //     panic!("Already wearing armor!");
                // }
                self.items.push(item);
            }
            ItemType::Ring => {
                // if self.items.iter().filter(|x| x.item_type == ItemType::Ring).count() == 2 {
                //     panic!("Can only wear 2 rings!");
                // }
                self.items.push(item);
            }
        }
    }

    fn remove_item(&mut self, item: &'a Item) {
        if let Some(index_item) = self.items.iter().enumerate().find(|x| x.1 == &item) {
            self.items.remove(index_item.0);
        }
    }

    fn clear_items(&mut self) {
        self.items = Vec::new();
    }

    fn recalculate_stats(&mut self) {
        self.armor = 0;
        self.damage = 0;
        for item in &self.items {
            self.armor += item.armor;
            self.damage += item.damage;
        }
    }
}

#[derive(PartialEq)]
enum ItemType {
    Weapon,
    Armor,
    Ring
}

#[derive(PartialEq)]
struct Item {
    name: String,
    damage: i32,
    armor: i32,
    cost: i32,
    item_type: ItemType
}

impl Item {
    fn new_weapon(name: &str, cost: i32, damage: i32) -> Item {
        Item { name: String::from(name), damage, cost, armor: 0, item_type: ItemType::Weapon }
    }
    
    fn new_armor(name: &str, cost: i32, armor: i32) -> Item {
        Item {name: String::from(name), armor, cost, damage: 0, item_type: ItemType::Armor }
    }

    fn new_ring(name: &str, cost: i32, damage: i32, armor: i32) -> Item {
        Item { name: String::from(name), damage, armor, cost, item_type: ItemType::Ring }
    }
}
