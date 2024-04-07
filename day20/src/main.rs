#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max, ops::Range};
// use fancy_regex::Regex;
// use regex::Regex;

const PRESENTS: u32 = 36000000;
const TENTH_PRESENTS: u32 = PRESENTS / 10;

fn main() {
    // going to assume it's at least house 100
    let mut house_num = 100;

    // going by every single house takes too long, try a more efficient search
    loop {
        if is_enough_presents(house_num) {
            let prev_checked_number = house_num / 2;
            let house_num = binary_search(prev_checked_number..house_num + 1);
            break;
        }
        //println!("Presents: {}", num_presents);
        house_num = house_num * 2;
    }

    println!("First house to surpass present num is {}", house_num);
    let num_presents = num_presents_for_house(house_num);
    println!("House {} gets {} presents", house_num, num_presents);
    println!("House {} gets {} presents", house_num - 1, num_presents_for_house(house_num - 1));
}

fn binary_search(range: Range<u32>) -> u32 {
    let range_length = range.end - range.start;
    if range_length == 1 {
        if is_enough_presents(range.start) {
            return range.start;
        }
        return range.end;
    }
    let halfway = range.start + range_length / 2;
    if is_enough_presents(halfway) {
        return binary_search(range.start..halfway);
    }
    else {
        return binary_search(halfway..range.end);
    }
}

fn is_enough_presents(house_number: u32) -> bool {
    let half_house_num = house_number / 2;
    // always divisible by 1 and itself
    let mut num_presents = 1 + house_number;
    for i in 2..=half_house_num {
        if house_number % i == 0 {
            num_presents += i;
        }
        if num_presents >= TENTH_PRESENTS {
            return true;
        }
    }
    num_presents >= TENTH_PRESENTS
}

fn num_presents_for_house(house_number: u32) -> u32 {
    let half_house_num = house_number / 2;
    // Always divisible by 1 and itself
    // (double counts for house 1, but that's fine)
    let mut num_presents: u32 = 1 + house_number;
    for i in 2..=half_house_num {
        if house_number % i == 0 {
            num_presents += i;
        }
    }
    // multiply by 10 at end
    num_presents * 10
}
