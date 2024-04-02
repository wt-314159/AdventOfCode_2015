use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};
use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut wires: HashMap<&str, Rc<RefCell<Element>>> = HashMap::new();
    let digits_regex = Regex::new("[0-9]+").unwrap();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let inputs_len = params.len();
        let drain_string = params.last().expect("No last string.");
        
        
        let source: Rc<RefCell<Element>> = match inputs_len {
            3 => {
                if digits_regex.captures_len() > 0 {
                    let val: u16 = params[0].parse::<u16>().expect(&format!("Failed to parse {} into u16", params[0]));
                    Rc::new(RefCell::new(Element::Input(val)))
                }
                else {
                    let wire_struct = WireStruct::mut_ref_new(params[0]);
                    let default_wire_element = Rc::new(RefCell::new(Element::Wire(wire_struct)));
                    let source_wire = wires.entry(params[0]).or_insert(default_wire_element);
                    Rc::clone(&source_wire)
                }
            }

            _ => panic!("Unexpected line length")
        };

        if !wires.contains_key(drain_string) {
            let default_wire_element = Rc::new(RefCell::new(Element::Wire(WireStruct::mut_ref_new(drain_string))));
            wires.insert(drain_string, default_wire_element);
        }

        let drain = wires.get(drain_string).expect("Failed to get drain");        
        let cloned = drain.clone();
        let drain_element = (*cloned).borrow_mut();
        drain_element.set_source(source);
    }
}

pub enum Element<'a> {
    Input(u16),
    Wire(Rc<RefCell<WireStruct<'a>>>),
    NotGate(Rc<RefCell<Element<'a>>>),
    LShiftGate(Rc<RefCell<Element<'a>>>),
    RShiftGate(Rc<RefCell<Element<'a>>>),
    AndGate(Rc<RefCell<Element<'a>>>, Rc<RefCell<Element<'a>>>),
    OrGate(Rc<RefCell<Element<'a>>>, Rc<RefCell<Element<'a>>>),
}

impl<'a> Element<'a> {
    pub fn provide_value(&self) -> u16 {
        match self {
            Self::Input(value) => *value,
            Self::Wire(wire) => match &(*(*wire)).borrow().source {
                Some(source) => (*source.borrow()).provide_value(),
                None => panic!("Wire doesn't have a source!")
            }
            Self::NotGate(source) => !(*source.borrow()).provide_value(),
            Self::LShiftGate(source) => (*source.borrow()).provide_value() << 2,
            Self::RShiftGate(source) => (*source.borrow()).provide_value() >> 2,
            Self::AndGate(source1, source2) => (*source1.borrow()).provide_value() & (*source2.borrow()).provide_value(),
            Self::OrGate(source1, source2) => (*source1.borrow()).provide_value() | (*source2.borrow()).provide_value(),
        }
    }

    pub fn set_source(&self, source: Rc<RefCell<Element<'a>>>) {
        if let Self::Wire(wire) = self {
            if let Some(_) = &wire.borrow().source {
                panic!("Wire source already set!")
            }
            (**wire).borrow_mut().source = Some(source);
        }
        else {
            panic!("Trying to set source on element other than wire!")
        }
    } 
}

pub struct WireStruct<'a> {
    id: &'a str,
    source: Option<Rc<RefCell<Element<'a>>>>
}

impl<'a> WireStruct<'a> {
    fn mut_ref_new(id: &str) -> Rc<RefCell<WireStruct>> {
        Rc::new(RefCell::new(WireStruct { id: id, source: None }))
    }
}