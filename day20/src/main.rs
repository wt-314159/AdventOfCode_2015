#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max, ops::Range, time::Instant};
// use fancy_regex::Regex;
// use regex::Regex;

const PRESENTS: u32 = 36000000;
const TENTH_PRESENTS: u32 = PRESENTS / 10;
const PRIMES: [u32; 168] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997];

fn main() {
    // going to assume it's at least house 128
    // start with a power of two as powers of two are likely to be highly divisible
    let mut house_num = 128;

    println!("First house to surpass present num is {}", house_num);
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
    // start at half_house_num and work backwards, as this 
    // will increase the num_presents count quicker and 
    // reduce the number of checks we need to do before
    // returning true (if total number of presents is greater
    // than PRESENTS)
    for i in (2..=half_house_num).rev() {
        if house_number % i == 0 {
            num_presents += i;
        }
        if num_presents >= TENTH_PRESENTS {
            return true;
        }
    }
    num_presents >= TENTH_PRESENTS
}

// turns out prime factorisation is pretty slow, I think because of all
// the memory allocation we're doing in the implementation, and the checks
// to ensure we don't duplicate any factors
// should only have to check if house-number is divisible by prime numbers
fn presents_from_primes(house_number: u32) -> u32 {
    // Find all prime factors
    let prime_factors = find_prime_factors(house_number);
    // find all factors from prime factors
    let all_factors = factors_from_prime_factors(prime_factors);
    // calculate num presents
    let mut num_presents = all_factors.iter().sum();
    num_presents *= 10;
    num_presents
}

fn find_prime_factors(number: u32) -> Vec<u32> {
    let mut prime_factors: Vec<u32> = Vec::new();
    let mut quotient = number;

    while quotient != 1 {
        for i in 0..PRIMES.len() {
            let prime = PRIMES[i];
            if quotient % prime == 0 {
                prime_factors.push(prime);
                quotient = quotient / prime;
                break;
            }
        }
    }

    prime_factors
}

fn factors_from_prime_factors(prime_factors: Vec<u32>) -> Vec<u32> {
    // include prime factors
    let mut all_factors = Vec::new();
    for i in 0..prime_factors.len() {
        let mut factor = prime_factors[i];
        if !all_factors.contains(&factor){
            all_factors.push(factor);
        }

        for j in i + 1..prime_factors.len() {
            factor = factor * prime_factors[j];
            if !all_factors.contains(&factor){
                all_factors.push(factor);
            }
        }
    }
    all_factors.push(1);
    all_factors
}

// Attempting to reduce the number of modulos we need to take to get a score for a house,
// by storing factors already worked out and using those later on, turned out much slower
fn num_presents_for_house_2(house_number: u32, factors: &mut HashMap<u32, (u32, Vec<u32>)>) -> u32 {
    let half_house_num = house_number / 2;
    let mut num_presents = 1 + house_number;
    let mut factor_indices: Vec<u32> = Vec::new();

    for i in (2..=half_house_num).rev() {
        // num presents already counted
        let mut already_counted = false;
        for indices in &factor_indices {
            if factors[&indices].1.contains(&i) {
                already_counted = true;
                break;
            }
        }
        if already_counted {
            continue;
        }
        if house_number % i == 0 {
            if factors.contains_key(&i) {
                // add the number of presents that factor got
                num_presents += factors[&i].0 - 1;
                factor_indices.push(i);
                continue;
            }
            num_presents += i;
        }
    }
    factors.insert(house_number, (num_presents, factor_indices));
    num_presents * 10
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

