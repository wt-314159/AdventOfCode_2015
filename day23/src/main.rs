#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;

fn main() {
    let input = fs::read_to_string("./src/puzzle_input.txt").expect("Failed to read input");
    //let input = fs::read_to_string("./src/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    let mut instructions = Vec::new();
    for line in input.split("\n") {
        let params: Vec<&str> = line.split_whitespace().collect();
        let instr = match params[0] {
            "hlf" => Instruction::hlf(index_from_reg(params[1])),
            "tpl" => Instruction::tpl(index_from_reg(params[1])),
            "inc" => Instruction::inc(index_from_reg(params[1])),
            "jmp" => Instruction::jmp(offset_from_input(params[1])),
            "jie" => Instruction::jie(index_from_reg(params[1]), offset_from_input(params[2])),
            "jio" => Instruction::jio(index_from_reg(params[1]), offset_from_input(params[2])),
            other => panic!("Unrecognised instruction: '{}'", other)
        };
        instructions.push(instr);
    }

    // let hlf = |r: &mut Register| r.0 /= 2;
    // let tpl = |r: &mut Register| r.0 *= 3;
    // let inc = |r: &mut Register| r.0 += 1;
    // let jmp = |i: i32| next_instruction += i;
    // let jie = |r: &Register, i: i32| if r.0 % 2 == 0 { next_instruction += i };
    // let jio = |r: &Register, i: i32| if r.0 == 1 { next_instruction += i};

    run_instructions(instructions);
    println!("Program finisehd");
}

fn index_from_reg(register: &str) -> usize {
    let trimmed = register.trim_matches(',');
    match trimmed {
        "a" => 0,
        "b" => 1,
        other => panic!("Register name not recognised {}", other)
    }
}

fn offset_from_input(input: &str) -> i32 {
    input.parse().expect(&format!("Failed to parse {}", input))
}

fn run_instructions(instructions: Vec<Instruction>) {
    // Part 2, for part 1, first register also starts with value of 0
    let mut registers = vec![Register(1), Register(0)];
    let mut next_instruction:i32 = 0;
    let instruction_len = instructions.len() as i32;

    loop {
        if next_instruction >= instruction_len || next_instruction < 0 {
            println!("Program ending as next instruction index is {}", next_instruction);
            break;
        }
        let instruction = &instructions[next_instruction as usize];
        let offset = handle_instruction(&mut registers, instruction);
        next_instruction += offset;
    }

    println!("Register a: '{}', Register b: '{}'", registers[0].0, registers[1].0);
}

fn handle_instruction(registers: &mut Vec<Register>, instr: &Instruction) -> i32 {
    let mut jump_offset = 1;
    match instr {
        Instruction::hlf(i) => half(&mut registers[*i]),
        Instruction::tpl(i) => triple(&mut registers[*i]),
        Instruction::inc(i) => increment(&mut registers[*i]),
        Instruction::jmp(offset) => jump_offset = *offset,
        Instruction::jie(i, offset) => jump_offset = jump_if_even(&registers[*i], *offset),
        Instruction::jio(i, offset) => jump_offset = jump_if_one(&registers[*i], *offset),
    }
    jump_offset
}

enum Instruction {
    hlf(usize),
    tpl(usize),
    inc(usize),
    jmp(i32),
    jie(usize, i32),
    jio(usize, i32)
}

fn half(r: &mut Register) {
    r.0 /= 2;
}

fn triple(r: &mut Register) {
    r.0 *= 3;
}

fn increment(r: &mut Register) {
    r.0 += 1;
}

fn jump_if_even(r: &Register, offset: i32) -> i32 {
    if r.0 % 2 == 0 {
        return offset;
    }
    1
}

fn jump_if_one(r: &Register, offset: i32) -> i32 {
    if r.0 == 1 {
        return offset;
    }
    1
}

struct Register(u32);