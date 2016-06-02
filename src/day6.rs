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

type GridPosition = (usize, usize);

fn parse_instruction(s: &str) -> Instruction {
    // TODO:
    // If I could get a tuple from a vector, I could do something like:
    // let (instruction_type, pos1, _, pos2) = raw_tuple
    // and then parse each item individually

    let raw: Vec<&str> = s.split_whitespace().collect();
    let pos1_raw: Vec<&str> = raw[1].split(",").collect();
    let pos2_raw: Vec<&str> = raw[3].split(",").collect();

    let pos1 = (
        pos1_raw[0].parse::<usize>().unwrap(),
        pos1_raw[1].parse::<usize>().unwrap()
    );
    let pos2 = (
        pos2_raw[0].parse::<usize>().unwrap(),
        pos2_raw[1].parse::<usize>().unwrap()
    );

    let instruction_type = match raw[0] {
       "turnon" => InstructionType::On,
       "turnoff" => InstructionType::Off,
       "toggle" => InstructionType::Toggle,
       _ => panic!("Unknown instruction type"),
    };

    Instruction { start: pos1, end: pos2, type_: instruction_type }
}

fn part1(instructions: &Vec<Instruction>) -> usize {
    let mut grid = [false; 1000000];

    for i in instructions {
        let action = &i.type_;
        let start_row = i.start.0;
        let end_row = i.end.0;
        let start_col = i.start.1;
        let end_col = i.end.1;

        for j in start_row..end_row+1 {
            let row = j * 1000;

            for k in start_col..end_col+1 {
                let pos = row + k;

                grid[pos] = match *action {
                    InstructionType::On => true,
                    InstructionType::Off => false,
                    InstructionType::Toggle => !grid[pos],
                };
            }
        }
    }

    grid.iter().filter(|&x| *x).count()
}

fn part2(instructions: &Vec<Instruction>) -> usize {
    let mut grid = [0; 1000000];

    for i in instructions {
        let action = &i.type_;
        let start_row = i.start.0;
        let end_row = i.end.0;
        let start_col = i.start.1;
        let end_col = i.end.1;

        for j in start_row..end_row+1 {
            let row = j * 1000;

            for k in start_col..end_col+1 {
                let pos = row + k;

                let light = grid[pos];

                match *action {
                    InstructionType::On => grid[pos] += 1,
                    InstructionType::Off => {
                        if light > 0 { grid[pos] -= 1; }
                    }
                    InstructionType::Toggle => grid[pos] += 2,
                };
            }
        }
    }

    grid.iter().fold(0, |brightness, light| brightness + light)
}

pub fn result() -> (usize, usize) {
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
fn test_parse_instruction() {
    let i = Instruction {
        start: (756, 53), end: (923, 339), type_: InstructionType::Off
    };
    assert_eq!(parse_instruction("turnoff 756,53 through 923,339"), i);
}

#[test]
fn test_result() {
    assert_eq!(result(), (569999, 17836115));
}
