#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    // Boss stats
    // Hit Points: 71
    // Damage: 10
    let mut boss = Entity::new(10, 71);
    let spells = spells_list();

    let mut me = Entity::new(0, 50);
    println!("Created spells");
}

fn spells_list() -> Vec<Spell> {
    let magic_missile = Spell::new_instant("Magic Missile", 53, |_, boss, _| boss.hit_points -= 4);
    let drain = Spell::new_instant("Drain", 73, |me, boss, _| {
        me.hit_points += 2;
        boss.hit_points -= 2;
    });
    let shield = Spell::new_effect("Shield", 113, 6,|me, _, i| {
        if i == 6 {
            me.armor += 7;
        }
        if i == 0 {
            me.armor -= 7;
        }
    });
    let poison = Spell::new_effect("Poison", 173, 6, |me, boss, i| boss.hit_points -= 3);
    let recharge = Spell::new_effect("Recharge", 229, 5, |me, boss, i| me.mana += 101);

    vec![magic_missile, drain, shield, poison, recharge]
}

fn minimum_mana_win(
    spells: &Vec<Spell>, 
    me: &mut Entity, 
    boss: &mut Entity, 
    least_mana: usize, 
    effects: &mut Vec<(usize, i32)>) -> usize {
    
    // apply effects at start of turn
    for effect in &mut *effects {
        
    }
    
    // Get spell that's within our mana budget
    for (i, spell) in spells.iter().enumerate()
    .filter(|x| x.1.mana_cost <= me.mana) {
        // Check effect isn't already applied
        if spell.duration != None && effects.iter().any(|x| x.0 == i) {
            continue;
        }
    }
    least_mana
}

struct Entity {
    hit_points: i32,
    damage: i32,
    armor: i32,
    mana: usize,
}

impl Entity {
    fn new(damage: i32, hit_points: i32) -> Entity {
        Entity { damage, hit_points, armor: 0, mana: 500 }
    }
}

struct Spell {
    name: String,
    mana_cost: usize,
    duration: Option<usize>,
    action: Box<dyn Fn(&mut Entity, &mut Entity, usize)>
}

impl Spell {
    fn new_instant<F>(name: &str, mana_cost: usize, action: F) -> Spell where 
    F: Fn(&mut Entity, &mut Entity, usize) + 'static {
        Spell { name: String::from(name), mana_cost, action: Box::new(action), duration: None }
    }

    fn new_effect<F>(name: &str, mana_cost: usize, duration: usize, action: F) -> Spell where 
    F: Fn(&mut Entity, &mut Entity, usize) + 'static {
        Spell {name: String::from(name), mana_cost, action: Box::new(action), duration: Some(duration) }
    }
}