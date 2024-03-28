use std::fs;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);

    let mut total_area = 0;
    let mut ribbon_length = 0;
    for line in input.split_whitespace() {
        let dimemsion_strings: Vec<&str> = line.split('x').collect();
        let mut dimensions: Vec<u32> = dimemsion_strings.iter().map(|x| x.parse::<u32>().unwrap()).collect();
        dimensions.sort();

        if dimensions.len() != 3 {
            panic!("Wrong number of dimensions! {:?}", dimensions);
        }
        
        let box_area = 3 * dimensions[0] * dimensions[1] + 2 * dimensions[1] * dimensions[2] + 2 * dimensions[0] * dimensions[2];
        let box_vol = dimensions[0] * dimensions[1] * dimensions[2];
        let smallest_circ = 2 * (dimensions[0] + dimensions[1]);

        total_area = total_area + box_area;
        ribbon_length = ribbon_length + smallest_circ + box_vol;
    }

    println!("Total area needed: {}", total_area);
    println!("Total ribbon needed: {}", ribbon_length);
}
