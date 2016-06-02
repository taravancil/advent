use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, PartialEq)]
enum InstructionType {
    On,
    Off,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    start: GridPosition,
    end: GridPosition,
    type_: InstructionType,
}

struct Light {
    on: bool,
}

type GridRow = [Light; 1000];
type Grid = [GridRow; 1000];
type GridPosition = (u32, u32);

fn parse_instruction(s: &str) -> Instruction {
    // TODO:
    // If I could get a tuple from a vector, I could do something like:
    // let (instruction_type, pos1, _, pos2) = raw_tuple
    // and then parse each item individually

    let raw: Vec<&str> = s.split_whitespace().collect();
    let pos1_raw: Vec<&str> = raw[1].split(",").collect();
    let pos2_raw: Vec<&str> = raw[3].split(",").collect();
    println!("{:?}", pos2_raw);

    let pos1 = (
        pos1_raw[0].parse::<u32>().unwrap(),
        pos1_raw[1].parse::<u32>().unwrap()
    );
    let pos2 = (
        pos2_raw[0].parse::<u32>().unwrap(),
        pos2_raw[1].parse::<u32>().unwrap()
    );

    let instruction_type = match raw[0] {
       "turnon" => InstructionType::On,
       "turnoff" => InstructionType::Off,
       "toggle" => InstructionType::Toggle,
       _ => panic!("Unknown instruction type"),
    };

    Instruction { start: pos1, end: pos2, type_: instruction_type }
}

#[allow(unused_variables)]
fn part1(instructions: &Vec<Instruction>) -> u32 {
    42
}

#[allow(unused_variables)]
fn part2(instructions: &Vec<Instruction>) -> u32 {
    42
}

pub fn result() -> (u32, u32) {
    let mut f = File::open("data/day6").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let instructions: Vec<Instruction> = input.split('\n')
        .map(|i| parse_instruction(&i))
        .collect();

    (part1(&instructions), part2(&instructions))
}

pub fn output() {
    let r = result();
    println!("-----\nDay 6\n-----\nPart 1: {}\nPart 2: {}\n", r.0, r.1);
}

#[test]
fn test_result() {
    assert_eq!(result(), (42, 42));
}
