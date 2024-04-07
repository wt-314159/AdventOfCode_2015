#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max, ops::Range, time::Instant};
// use fancy_regex::Regex;
// use regex::Regex;

const PRESENTS: u32 = 36000000;
const TENTH_PRESENTS: u32 = PRESENTS / 10;
const ELEVENTH_PRESENTS: u32 = PRESENTS / 11;
const PRIMES: [u32; 168] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997];

fn 
main() {
    // Part 1 answer: 831600
    
    // trying naive implementation
    // Part 1
    let house_num = step_by_step(1_000_000, TENTH_PRESENTS as usize);
    if let Some(house) = house_num {
        println!("First house to pass is {}, with {} presents", house.0, house.1);
    }
    println!("No house found passing limit");
}

// lower the upper limit of a search and hopefully narrow in on the answer
fn narrowing_search(range: Range<u32>, increment: u32) -> u32 {
    println!("Searching range {} - {}, increment: {}", range.start, range.end, increment);
    let mut house_num = range.start;
    let mut incr = increment;
    if increment > 3 {
        incr = increment - 3;
    }
    let increments = vec![1,2,incr];
    let mut incrmnt_index = 0;
    while house_num <= range.end {
        if is_enough_presents(house_num) {
            println!("Upper limit narrowed to {}", house_num);
            if increment == 1 {
                return house_num;
            }
            return narrowing_search(range.start..house_num, increment / 2);
        }
        if increment > 2 {
            house_num += increments[incrmnt_index];
            incrmnt_index += 1;
            if incrmnt_index == increments.len() {
                incrmnt_index = 0;
            }
        }
        else {
            house_num += increment;
        }
    }
    println!("Upper limit not narrowed");
    return narrowing_search(range, increment / 2);
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
    for i in (2..=half_house_num) {
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

// Seems like a naive implementation, but actually orders of magnitude faster
// Partly because addition is much quicker than division, but also because we 
// don't repeat our work, if we try and find the factors for each number, we
// have to divide by every number up to half of that number, and then for the
// next number, we divide by all the same numbers but go one further.
// In this method though, for each number we would otherwise divide by, we add
// it's score to all the relevant houses in just one one pass, instead of many
// passes of division.
fn step_by_step(max_house: usize, limit: usize) -> Option<(usize, usize)> {
    let mut houses: Vec<usize> = [0].repeat(max_house);
    for i in 1..houses.len() {
        for house in (i-1..houses.len()).step_by(i) {
            houses[house] += i;
        }
    }
    println!("All presents delivered to houses");
    let house  = houses.iter().enumerate().find(|x| x.1 >= &limit);

    if let Some((house_num, presents)) = house {
        return Some((house_num + 1, *presents));    // indices are off by 1
    }
    else {
        return None
    }
}
