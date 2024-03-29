use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut wires: HashMap<&str, Wire> = HashMap::new();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let inputs_len = params.len();
        let drain_string = params.last().expect("No last string.");
        
        let mut drain: &mut Wire;
        let mut default = Wire {id: drain_string, source: None};
        if let Some(d) = wires.get_mut(drain_string) {
            drain = d;
        }
        else {
            drain = &mut default;
        }
        
        // source must be a single value
        if params.len() == 3 {
            let val: u16 = params[0].parse::<u16>().expect(&format!("Failed to parse {} into u16", params[0]));
            drain.from_value(&Input{value: val});
        }
        // each wire can only have one source, so if it's
        // already in the hashmap, it must be as a source
        // rather than as the drain for some other source
        if wires.contains_key(drain_string) {
            // Create the necessary Gate and values first
            for param_str in 0..inputs_len {
                
            }
        }
    }
}

pub trait Element {
    fn provide_value(&self) -> u16;
}

struct Wire<'a> {
    pub id: &'a str,
    pub source: Option<&'a dyn Element>
}

impl<'a> Element for Wire<'a> {
    fn provide_value(&self) -> u16 {
        self.source.expect("No input to wire!").provide_value()
    }
}

impl<'a> Wire<'a> {
    pub fn from_value(&mut self, input: &'a Input) {
        self.source = Some(input);
    }
}

#[derive(Clone, Copy)]
struct Input {
    value: u16
}

impl Element for Input {
    fn provide_value(&self) -> u16 {
        self.value
    }
}

#[allow(non_camel_case_types)]
struct AND_Gate<'a> {
    source_1: &'a dyn Element,
    source_2: &'a dyn Element
}

impl<'a> Element for AND_Gate<'a> {
    fn provide_value(&self) -> u16 {
        let val1 = self.source_1.provide_value();
        let val2 = self.source_2.provide_value();
        val1 & val2
    }
}

#[allow(non_camel_case_types)]
struct NOT_Gate<'a> {
    source: &'a dyn Element
}

impl<'a> Element for NOT_Gate<'a> {
    fn provide_value(&self) -> u16 {
        !self.source.provide_value()
    }
}

#[allow(non_camel_case_types)]
struct OR_Gate<'a> {
    source_1: &'a dyn Element,
    source_2: &'a dyn Element
}

impl<'a> Element for OR_Gate<'a> {
    fn provide_value(&self) -> u16 {
        let val1 = self.source_1.provide_value();
        let val2 = self.source_2.provide_value();
        val1 | val2
    }
}

#[allow(non_camel_case_types)]
struct LSHIFT_Gate<'a> {
    source: &'a dyn Element
}

impl<'a> Element for LSHIFT_Gate<'a> {
    fn provide_value(&self) -> u16 {
        self.source.provide_value() << 2
    }
}

#[allow(non_camel_case_types)]
struct RSHIFT_Gate<'a> {
    source: &'a dyn Element
}

impl<'a> Element for RSHIFT_Gate<'a> {
    fn provide_value(&self) -> u16 {
        self.source.provide_value() >> 2
    }
}

// have some structs that implement a trait with a provide_value() method
// wires will have a single source, and provide_value() will just call 
// provide_value() on their source, gates with have one or two sources
// depending on the gate (NOT gate will have one, AND and OR etc will have
// 2), and will call provide_value() on both their sources, and then perform
// the relevant operation on those values.
// Then we just need to build a connected web of these structs based on the
// file provided, and finally we just call provice_value() on the one we're 
// interested in
