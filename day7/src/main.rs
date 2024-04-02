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
                // Part 2: Override value of 'b' wire to previous value of 'a' wire
                // (Replace below if/else statement with code from else statement for
                //  Part 1 answer.)
                if (drain_string == &"b") {
                    parse_to_input_or_wire(&digits_regex, &mut wires, "3176")
                }
                else {
                    parse_to_input_or_wire(&digits_regex, &mut wires, params[0])
                }
            }
            4 => { 
                // must be a not gate
                let gate_source = parse_to_input_or_wire(&digits_regex, &mut wires, params[1]);
                Element::NotGate(gate_source).get_boxed_ref()
            }
            5 => {
                let gate_source1 = parse_to_input_or_wire(&digits_regex, &mut wires, params[0]);
                let gate_source2 = parse_to_input_or_wire(&digits_regex, &mut wires, params[2]);
                let param2_int = params[2].parse::<u16>();

                // could be AND, OR, LSHIFT, or RSHIFT gates
                match params[1] {
                    "AND" => Element::AndGate(gate_source1, gate_source2).get_boxed_ref(),
                    "OR" => Element::OrGate(gate_source1, gate_source2).get_boxed_ref(),
                    "LSHIFT" => Element::LShiftGate(gate_source1, param2_int.expect(&format!("Failed to parse {} to u16", params[2]))).get_boxed_ref(),
                    "RSHIFT" => Element::RShiftGate(gate_source1, param2_int.expect(&format!("Failed to parse {} to u16", params[2]))).get_boxed_ref(),
                    other => panic!("Unexpected gate name! {}", other)
                }
            }

            len => panic!("Unexpected line length! Length: {}", len)
        };

        if !wires.contains_key(drain_string) {
            let default_wire_element = WireStruct::default_wire_element(drain_string);
            wires.insert(drain_string, default_wire_element);
        }

        let drain = wires.get(drain_string).expect("Failed to get drain");        
        let cloned = drain.clone();
        let drain_element = (*cloned).borrow_mut();
        drain_element.set_source(source);
    }

    println!("Processing complete!");

    println!("Computing");
    let mut wire_values: HashMap<&str, u16> = HashMap::new();
    let result = wires.get("a").expect("Failed to get wire 'a'").borrow().provide_value(&mut wire_values);
    println!("Value of a: {}", result);
}

fn parse_to_input_or_wire<'a>(
    digits_regex: &Regex,
    wires: &mut HashMap<&'a str, Rc<RefCell<Element<'a>>>>, 
    param: &'a str) -> Rc<RefCell<Element<'a>>> {
        if digits_regex.is_match(param) {
            let val: u16 = param.parse::<u16>().expect(&format!("Failed to parse {} into u16", param));
            Rc::new(RefCell::new(Element::Input(val)))
        }
        else {
            let source_wire = get_wire_or_default(wires, param);
            Rc::clone(&source_wire)
        }
}

fn get_wire_or_default<'a>(wires: &mut HashMap<&'a str, Rc<RefCell<Element<'a>>>>, id: &'a str) -> Rc<RefCell<Element<'a>>> {
    let default_wire_element = WireStruct::default_wire_element(id);
    let entry = wires.entry(id).or_insert(default_wire_element);
    Rc::clone(&entry)
}

pub enum Element<'a> {
    Input(u16),
    Wire(Rc<RefCell<WireStruct<'a>>>),
    NotGate(Rc<RefCell<Element<'a>>>),
    LShiftGate(Rc<RefCell<Element<'a>>>, u16),
    RShiftGate(Rc<RefCell<Element<'a>>>, u16),
    AndGate(Rc<RefCell<Element<'a>>>, Rc<RefCell<Element<'a>>>),
    OrGate(Rc<RefCell<Element<'a>>>, Rc<RefCell<Element<'a>>>),
}

impl<'a> Element<'a> {
    pub fn provide_value(&self, wire_values: &mut HashMap<&'a str, u16>) -> u16 {
        //print!(".");
        match self {
            Self::Input(value) => *value,
            Self::Wire(wire) => match &wire.borrow().source {
                Some(source) => { 
                    let wire_id = &wire.borrow().id;
                    if wire_values.contains_key(wire_id) {
                        wire_values[wire_id]
                    }
                    else {
                        let val = source.borrow().provide_value(wire_values);
                        wire_values.insert(wire_id, val);
                        val
                    }
                }
                None => panic!("Wire {} doesn't have a source!", wire.borrow().id)
            }
            Self::NotGate(source) => !(source.borrow()).provide_value(wire_values),
            Self::LShiftGate(source, val) => (source.borrow()).provide_value(wire_values) << val,
            Self::RShiftGate(source, val) => (source.borrow()).provide_value(wire_values) >> val,
            Self::AndGate(source1, source2) => (source1.borrow()).provide_value(wire_values) & (source2.borrow()).provide_value(wire_values),
            Self::OrGate(source1, source2) => (source1.borrow()).provide_value(wire_values) | (source2.borrow()).provide_value(wire_values),
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

    pub fn get_boxed_ref(self) -> Rc<RefCell<Element<'a>>> {
        Rc::new(RefCell::new(self))
    }
}

pub struct WireStruct<'a> {
    #[allow(dead_code)]
    id: &'a str,        // Could be useful for debugging or later on
    source: Option<Rc<RefCell<Element<'a>>>>
}

impl<'a> WireStruct<'a> {
    fn default_wire_element(id: &str) -> Rc<RefCell<Element>> {
        let element = Element::Wire(Rc::new(RefCell::new(WireStruct { id: id, source: None })));
        Rc::new(RefCell::new(element))
    }
}