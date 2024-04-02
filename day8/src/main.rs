use std::fs;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut code_chars = 0;
    let mut string_chars = 0;
    let mut encoded_chars = 0;
    for line in input.split_whitespace() {
        let mut skip = 0;
        let mut diff = 2;           // add 2 diffs for quotes at start and end of line
        let mut encoded_extra = 4;    // add 2 extra encoded chars for each double quote
        code_chars += line.len();

        for (i, c) in line.chars().enumerate().skip(1) {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if c == '\\' {
                let next = &line[i + 1..i + 2];
                if next == "x" {
                    diff += 3;
                    skip = 3;
                    encoded_extra += 1;
                }
                else {
                    diff += 1;
                    skip = 1;
                    encoded_extra += 2;
                }
            }
        }
        string_chars += line.len() - diff;
        encoded_chars += line.len() + encoded_extra;
    }

    println!("Difference is: {}", code_chars - string_chars);
    println!("Extra chars in encoded is: {}", encoded_chars - code_chars);
}
