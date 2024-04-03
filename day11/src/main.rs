use fancy_regex::Regex;

const ALPHABET: [char; 26] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

fn main() {
    // Part 1
    // let mut password = String::from("cqjxjnds");
    // Part 2, use previous answer as input for part 2
    let mut password = String::from("cqjxxyzz");
    let checker = PasswordChecker::new();

    let mut counter = 0;
    loop {
        password = increment(&password);
        counter += 1;
        if checker.is_valid(&password) {
            break;
        }
        if counter > 1000000 {
            break;
        }
    }

    println!("Found next password after {} iterations", counter);
    println!("Next valid password: {}", password);
}

fn increment(password: &str) -> String {
    let mut new_chars: Vec<char> = password.chars().collect();
    for i in 0..password.len() {
        let index = password.len() - 1 - i;
        let c = new_chars[index];
        let char_index = ALPHABET.iter()
            .position(|&x| x == c)
            .expect(&format!("Character '{}' not in alphabet", c));

        if char_index < ALPHABET.len() - 1 {
            let new_c = ALPHABET[char_index + 1];
            new_chars[index] = new_c;
            break;  // stop here
        }
        else {
            let new_c = ALPHABET[0];
            new_chars[index] = new_c;
        }
    }
    new_chars.into_iter().collect()
}

struct PasswordChecker {
    forbidden_chars_regex: Regex,
    asc_chars_regex: Regex,
    repeated_chars_regex: Regex,
}

impl PasswordChecker {
    fn new() -> PasswordChecker {
        PasswordChecker { 
            forbidden_chars_regex: Regex::new("[oil]").unwrap(), 
            asc_chars_regex: Regex::new("abc|bcd|cde|def|efg|fgh|ghi|hij|ijk|jkl|klm|lmn|mno|nop|opq|pqr|qrs|rst|stu|tuv|uvw|vwx|wxy|xyz").unwrap(),
            repeated_chars_regex: Regex::new("([a-z])\\1.*([a-z])\\2").unwrap()
        }
    }

    fn is_valid(&self, password: &str) -> bool {
        // Could probably combine all the regex, or use a RegexSet, 
        // just doing this quickly though so no real need to optimise
        !self.forbidden_chars_regex.is_match(password).expect("Failed on forbidden chars check") && 
        self.asc_chars_regex.is_match(password).expect("Failed on ascending chars check") &&
        self.repeated_chars_regex.is_match(password).expect("Failed on repeated chars check")

    }
}