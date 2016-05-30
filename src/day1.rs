use std::io::prelude::*;
use std::fs::File;

pub fn part1(input: &str) -> i32 {
    input.chars()
        .map(|c| up_or_down(c))
        .fold(0, |floor,x| floor + x)
}

pub fn part2(input: &str) -> usize {
    let mut floor = 0;

    // Use .enumerate() to track our position in input
    for (i, j) in input.chars().enumerate() {
        floor += up_or_down(j);

        // Instructions begin at position 1, not zero
        if floor == -1 { return i + 1 }
    }
    0
}

fn up_or_down(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _   => 0,
    }
}

pub fn result() -> (i32, usize) {
    let mut f = File::open("data/day1").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    (part1(&input), part2(&input))
}

pub fn output() {
    let r = result();
    println!("-----\nDay 1\n-----\nPart 1: {}\nPart 2: {}\n", r.0, r.1);
}

#[test]
fn test_up_or_down() {
    assert_eq!(up_or_down('('), 1);
    assert_eq!(up_or_down(')'), -1);
    assert_eq!(up_or_down('x'), 0);
}

#[test]
pub fn test_result() {
    assert_eq!(result(), (232, 1783));
}
