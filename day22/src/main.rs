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
    me.mana = 500;
    println!("Created spells");

    let mut effects: Vec<(usize, usize)> = Vec::new();
    let mut spells_cast: Vec<usize> = Vec::new();
    let min_mana = minimum_mana_win(&spells, &mut me, &mut boss, 10000, 0, &mut effects, &mut spells_cast);
    println!("Best solution found used {} mana", min_mana);
}

fn spells_list() -> Vec<Spell> {
    let magic_missile = Spell::new_instant(
        "Magic Missile", 
        53, 
        |_, boss, _| boss.hit_points -= 4, 
        |_, boss, _| boss.hit_points += 4);
    let drain = Spell::new_instant("Drain", 73, 
    |me, boss, _| {
            me.hit_points += 2;
            boss.hit_points -= 2;
        },
    |me, boss, _| {
            me.hit_points -= 2;
            boss.hit_points += 2;
        } );
    let shield = Spell::new_effect("Shield", 113, 6,
    |me, _, i| {
        if i == 6 {
            me.armor += 7;
        }
        if i == 1 {
            me.armor -= 7;
        }
        },
    |me, _, i| {
        if i == 1 {
            me.armor += 7;
        }
        if i == 6 {
            me.armor -= 7;
        }
    });
    let poison = Spell::new_effect("Poison", 173, 6, 
    |_, boss, _| boss.hit_points -= 3,
    |_, boss, _| boss.hit_points += 3);
    let recharge = Spell::new_effect("Recharge", 229, 5, 
    |me, _, _| me.mana += 101,
    |me, _, _| me.mana -= 101);

    vec![magic_missile, drain, shield, poison, recharge]
}

fn minimum_mana_win(
    spells: &Vec<Spell>, 
    me: &mut Entity, 
    boss: &mut Entity, 
    mut least_mana: i32, 
    mut mana_used: i32,
    effects: &mut Vec<(usize, usize)>,
    spells_cast: &mut Vec<usize>) -> i32 {

    // Part 2 - remove next section for Part 1
    // hard difficulty, take 1 hp damage each turn
    me.hit_points -= 1;
    if me.hit_points < 1 {
        me.hit_points += 1;
        return least_mana;
    }
    
    // apply effects at start of turn
    let removed_effects1 = apply_effects(spells, me, boss, effects);
    if boss.hit_points <= 0 && mana_used < least_mana {
        least_mana = mana_used;
        // undo effects
        undo_effects(spells, me, boss, effects, &removed_effects1);
        print_solution_found(spells, spells_cast, mana_used);
        return least_mana;
    }
    
    let my_mana = me.mana;
    // Get spell that's within our mana budget
    for (i, spell) in spells.iter().enumerate()
    .filter(|x| x.1.mana_cost <= my_mana) {
        // Check effect isn't already applied
        if spell.duration != None && effects.iter().any(|x| x.0 == i) {
            continue;
        }

        // If the spell would take us over the least mana used, no need to check it
        if mana_used + spell.mana_cost >= least_mana {
            continue;
        }

        // add spell to list of effects if it's an effects spell,
        // otherwise do the action instantly
        me.mana -= spell.mana_cost;
        mana_used += spell.mana_cost;
        spells_cast.push(i);
        if spell.duration != None {
            effects.push((i, spell.duration.unwrap()));
        }
        else {
            (spell.action)(me, boss, 0);
        }

        // Check if we've won
        if boss.hit_points <= 0 && mana_used < least_mana {
            print_solution_found(spells, spells_cast, mana_used);
            least_mana = mana_used;
            // undo turn 
            (spell.undo_action)(me, boss, 0);
            undo_effects(spells, me, boss, effects, &removed_effects1);
            me.mana += spell.mana_cost;
            spells_cast.pop();
            return least_mana;
        }

        // boss's turn
        // apply effects at start of turn
        let removed_effects2 = apply_effects(spells, me, boss, effects);
        // check if we've won
        if boss.hit_points <= 0 && mana_used < least_mana {
            print_solution_found(spells, spells_cast, mana_used);
            least_mana = mana_used;
            undo_effects(spells, me, boss, effects, &removed_effects2);
            if spell.duration == None {
                (spell.undo_action)(me, boss, 0);
            }
            undo_effects(spells, me, boss, effects, &removed_effects1);
            me.mana += spell.mana_cost;
            spells_cast.pop();
            return least_mana;
        }

        let mut damage_taken = boss.damage - me.armor;
        if damage_taken < 1 {
            damage_taken = 1;
        }
        me.hit_points -= damage_taken;

        // go on to next turn if we didn't die
        if me.hit_points > 0 {
            least_mana = minimum_mana_win(spells, me, boss, least_mana, mana_used, effects, spells_cast);
        }

        // unwind the effects of this turn, so next option isn't affected
        me.hit_points += damage_taken;
        me.mana += spell.mana_cost;
        mana_used -= spell.mana_cost;
        spells_cast.pop();

        undo_effects(spells, me, boss, effects, &removed_effects2);
        // undo instant spell
        if spell.duration == None {
            (spell.undo_action)(me, boss, 0);
        }
    }
    // out of options, undo effects from turn start and return least mana
    undo_effects(spells, me, boss, effects, &removed_effects1);
    // Part 2, remove following line for Part 1
    me.hit_points += 1;
    least_mana
}

fn print_solution_found(spells: &Vec<Spell>, spells_cast: &Vec<usize>, mana_used: i32) {
    print!("Won using {} mana: ", mana_used);
    let first_spell = &spells[spells_cast[0]];
    print!("{}", first_spell.name);
    for spell_index in spells_cast.iter().skip(1) {
        let spell = &spells[*spell_index];
        print!(" -> {}", spell.name);
    }
    println!("\n------------------------------------------------------------");
}

fn apply_effects(
    spells: &Vec<Spell>, 
    me: &mut Entity, 
    boss: &mut Entity, 
    effects: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {

        let mut to_remove = Vec::new();
        let mut removed: Vec<(usize, usize)> = Vec::new();
        for i in 0..effects.len() {
            let (spell_index, timer) = effects[i];
            let effect_spell = &spells[spell_index];
            (effect_spell.action)(me, boss, timer);
            if timer == 1 {
                to_remove.push(i);
            }
            else {
                effects[i].1 -= 1;
            }
        }
        // sort in descending order of index, as removing later indices doesn't affect
        // earlier indices, but removing early indices affects later indices
        to_remove.sort_by(|a, b| b.cmp(a));
        for i in to_remove {
            // remove spell and add it to list of removed spells
            removed.push(effects.remove(i));
        }
        removed
    }

fn undo_effects(
    spells: &Vec<Spell>,
    me: &mut Entity,
    boss: &mut Entity,
    effects: &mut Vec<(usize, usize)>,
    removed_effects: &Vec<(usize, usize)>) {
        // increment all timers, if this increases them over the spell's duration, remove them
        for i in 0..effects.len() {
            effects[i].1 += 1;
        }
        
        // add back in any spells previously removed
        // they are removed with timer of 1, so don't need to increment
        for removed in removed_effects {
            effects.push(*removed);
        }
        
        // do undo action for each spell 
        for i in 0..effects.len() {
            let effect = effects[i];
            let spell = &spells[effect.0];
            (spell.undo_action)(me, boss, effect.1);
        }

        // find any that should now be removed, as their timer equals their duration
        let mut to_remove = Vec::new();
        for i in 0..effects.len() {
            let (spell_index, _timer) = effects[i];
            let duration = spells[spell_index].duration.unwrap();
            if effects[i].1 == duration {
                to_remove.push(i);
            }
        }
        
        // remove any which need to be removed
        to_remove.sort_by(|a, b| b.cmp(a));
        for i in to_remove {
            effects.remove(i);
        }
    }

struct Entity {
    hit_points: i32,
    damage: i32,
    armor: i32,
    mana: i32,
}

impl Entity {
    fn new(damage: i32, hit_points: i32) -> Entity {
        Entity { damage, hit_points, armor: 0, mana: 500 }
    }
}

struct Spell {
    name: String,
    mana_cost: i32,
    duration: Option<usize>,
    action: Box<dyn Fn(&mut Entity, &mut Entity, usize)>,
    undo_action: Box<dyn Fn(&mut Entity, &mut Entity, usize)>
}

impl Spell {
    fn new_instant<F, G>(name: &str, mana_cost: i32, action: F, undo_action: G) -> Spell where 
    F: Fn(&mut Entity, &mut Entity, usize) + 'static,
    G: Fn(&mut Entity, &mut Entity, usize) + 'static {
        Spell { name: String::from(name), mana_cost, action: Box::new(action), undo_action: Box::new(undo_action), duration: None }
    }

    fn new_effect<F, G>(name: &str, mana_cost: i32, duration: usize, action: F, undo_action: G) -> Spell where 
    F: Fn(&mut Entity, &mut Entity, usize) + 'static,
    G: Fn(&mut Entity, &mut Entity, usize) + 'static {
        Spell {name: String::from(name), mana_cost, action: Box::new(action), undo_action: Box::new(undo_action), duration: Some(duration) }
    }
}