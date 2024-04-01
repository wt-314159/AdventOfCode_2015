use std::{collections::HashMap, rc::Rc, cell::RefCell, fs};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    
    let mut wires: HashMap<&str, Rc<Wire>> = HashMap::new();
    let digits_regex = Regex::new("[0-9]+").unwrap();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let inputs_len = params.len();
        let drain_string = params.last().expect("No last string.");

        let source: Rc<dyn Element> = match params.len() {
            3 => {
                if digits_regex.captures_len() > 0 {
                    let val: u16 = params[0].parse::<u16>().expect(&format!("Failed to parse {} into u16", params[0]));
                    Rc::new(Input {value: val})
                }
                else {
                    let source_wire = wires.entry(params[0]).or_insert(Rc::new(Wire {id: params[0], source: None }));
                    source_wire
                }
            }

            _ => panic!("Unexpected line length")
        };

        if !wires.contains_key(drain_string) {
            wires.insert(drain_string, Rc::new(Wire {id: drain_string, source: None }));
        }
        let mut drain = wires.get_mut(drain_string).expect("Failed to get wire.");
        drain.source = Some(source);
    }
}

pub enum Element {
    Input(u16),
    Wire(Box<Element>),
    NotGate(Box<Element>),
    LShiftGate(Box<Element>),
    RShiftGate(Box<Element>),
    AndGate(Box<Element>, Box<Element>),
    OrGate(Box<Element>, Box<Element>),
}

impl Element {
    pub fn provide_value(&self) -> u16 {
        match self {
            Self::Input(value) => *value,
            Self::Wire(source) => source.provide_value(),
            Self::NotGate(source) => !source.provide_value(),
            Self::LShiftGate(source) => source.provide_value() << 2,
            Self::RShiftGate(source) => source.provide_value() >> 2,
            Self::AndGate(source1, source2) => source1.provide_value() & source2.provide_value(),
            Self::OrGate(source1, source2) => source1.provide_value() | source2.provide_value()
        }
    }
}

// pub trait Element {
//     fn provide_value(&self) -> u16;
// }

// struct Wire<'a> {
//     pub id: &'a str,
//     pub source: Option<Rc<RefCell<dyn Element>>>
// }

// impl<'a> Element for Wire<'a> {
//     fn provide_value(&self) -> u16 {
//         self.source.as_ref().expect("No input to wire!").borrow().provide_value()
//     }
// }

// #[derive(Clone, Copy)]
// struct Input {
//     value: u16
// }

// impl Element for Input {
//     fn provide_value(&self) -> u16 {
//         self.value
//     }
// }

// #[allow(non_camel_case_types)]
// struct AND_Gate<'a> {
//     source_1: &'a dyn Element,
//     source_2: &'a dyn Element
// }

// impl<'a> Element for AND_Gate<'a> {
//     fn provide_value(&self) -> u16 {
//         let val1 = self.source_1.provide_value();
//         let val2 = self.source_2.provide_value();
//         val1 & val2
//     }
// }

// #[allow(non_camel_case_types)]
// struct NOT_Gate<'a> {
//     source: &'a dyn Element
// }

// impl<'a> Element for NOT_Gate<'a> {
//     fn provide_value(&self) -> u16 {
//         !self.source.provide_value()
//     }
// }

// #[allow(non_camel_case_types)]
// struct OR_Gate<'a> {
//     source_1: &'a dyn Element,
//     source_2: &'a dyn Element
// }

// impl<'a> Element for OR_Gate<'a> {
//     fn provide_value(&self) -> u16 {
//         let val1 = self.source_1.provide_value();
//         let val2 = self.source_2.provide_value();
//         val1 | val2
//     }
// }

// #[allow(non_camel_case_types)]
// struct LSHIFT_Gate<'a> {
//     source: &'a dyn Element
// }

// impl<'a> Element for LSHIFT_Gate<'a> {
//     fn provide_value(&self) -> u16 {
//         self.source.provide_value() << 2
//     }
// }

// #[allow(non_camel_case_types)]
// struct RSHIFT_Gate<'a> {
//     source: &'a dyn Element
// }

// impl<'a> Element for RSHIFT_Gate<'a> {
//     fn provide_value(&self) -> u16 {
//         self.source.provide_value() >> 2
//     }
// }

// have some structs that implement a trait with a provide_value() method
// wires will have a single source, and provide_value() will just call 
// provide_value() on their source, gates with have one or two sources
// depending on the gate (NOT gate will have one, AND and OR etc will have
// 2), and will call provide_value() on both their sources, and then perform
// the relevant operation on those values.
// Then we just need to build a connected web of these structs based on the
// file provided, and finally we just call provice_value() on the one we're 
// interested in
