fn main() {
    let mut input = String::from("1321131112");
    println!("Initial input: {}", input);

    // for part 1, iterate 40 times, not 50
    #[allow(unused_variables)]
    for i in 0..50 {
        input = look_and_say(&input);
        //println!("Result after {} iterations: {}", i + 1, input);
    }
    // result is too long to print, not much point printing it anyway
    //println!("End result: {}", input);
    println!("Lengths of result: {}", input.len());
}

fn look_and_say(input: &str) -> String {
    let mut prev_char = '\0';
    let mut char_count = 0;
    let mut result: String = String::new();
    // code here could be a bit neater, could probably also
    // do this with Regex's, just trying to code it quickly,
    // not create the most efficient or cleanest code
    for (i, c) in input.chars().enumerate() {
        if c == prev_char {
            char_count += 1;
        }
        else {
            if i != 0 {
                result.push_str(&char_count.to_string());
                result.push(prev_char);
            }
            char_count = 1;
        }
        if i == input.len() - 1 {
            result.push_str(&char_count.to_string());
            result.push(c);
        }

        prev_char = c;
    }
    result
}
