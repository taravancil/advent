use std::io::prelude::*;
use std::fs::File;

enum InstructionType {
    On,
    Off,
    Toggle,
}

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
