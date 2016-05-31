use std::io::prelude::*;
use std::fs::File;

#[allow(unused_variables)]
pub fn part1(input: &str) -> u32 {
    42
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> u32 {
    42
}

pub fn result() -> (u32, u32) {
    let mut f = File::open("data/day5").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    (part1(&input), part2(&input))
}

pub fn output() {
    let r = result();
    println!("-----\nDay 5\n-----\nPart 1: {}\nPart 2: {}\n", r.0, r.1);
}

#[test]
fn test_result() {
    assert_eq!(result(), (42, 42));
}
