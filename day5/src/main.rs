use std::fs;

const VOWELS: &str = "aeiou";

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut nice_counter = 0;
    for line in input.split_whitespace() {
        if is_nice_two(line) {
            nice_counter += 1;
        }
    }

    println!("Nice strings: {}", nice_counter);
}

// This method would be easier and probably quicker with regex,
// just didn't want to spend the time to think about how to do it
// with Regex
fn is_nice(str: &str) -> bool {
    let mut prev_char = '\0';
    let mut vowel_count = 0;
    let mut double_char = false;

    for c in str.chars() {
        if VOWELS.contains(c) {
            vowel_count += 1;
        }
        if c == prev_char {
            double_char = true;
        }
        if c == 'b' && prev_char == 'a' || 
            c == 'd' && prev_char == 'c' || 
            c == 'q' && prev_char == 'p' ||
            c == 'y' && prev_char == 'x' {
            
            return false;
        }
        prev_char = c;
    }

    return vowel_count > 2 && double_char;
}

// Not sure how to do this one with Regex, might have had to be 
// some combination of Regex and code, would definitely have taken
// me a while to figure out
fn is_nice_two(str: &str) -> bool {
    let mut prev_char = '\0';
    let mut two_prev_char = '\n';
    let mut prev_char_tuple: (char, char) = ('1', '2');
    let mut two_pairs = false;
    let mut third_wheel = false;
    let mut pairs: Vec<(char,char)> = Vec::new();

    for c in str.chars() {
        let char_tuple = (prev_char, c);
        if pairs.contains(&char_tuple) {
            two_pairs = true;
        }
        pairs.push(prev_char_tuple);

        if c == two_prev_char {
            third_wheel = true;
        }

        two_prev_char = prev_char;
        prev_char = c;
        prev_char_tuple = char_tuple;
    }
    return two_pairs && third_wheel;
}
