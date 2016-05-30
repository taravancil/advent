use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn update_position(c: char, p: (i32, i32)) -> (i32, i32) {
    match c {
        '>' => (p.0 + 1, p.1),
        '<' => (p.0 - 1, p.1),
        '^' => (p.0, p.1 + 1),
        'v' => (p.0, p.1 - 1),
        _   => p,
    }
}

pub fn part1(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut position = (0, 0);

    // Santa delivers a present to the initial house
    visited.insert(position);

    for c in input.chars() {
        position = update_position(c, position);
        visited.insert(position);
    }
    visited.len()
}

pub fn part2(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut position = (0, 0);
    let mut robo_position = (0, 0);

    visited.insert(position);

    for (i, c) in input.chars().enumerate() {
        // Santa's turn
        if i % 2 == 0 || i == 0 {
            position = update_position(c, position);
            visited.insert(position);
        }
        // Robo Santa's turn
        else {
            robo_position = update_position(c, robo_position);
            visited.insert(robo_position);
        }
    }
    visited.len()
}

pub fn result() -> (usize, usize) {
    let mut f = File::open("data/day3").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    (part1(&input), part2(&input))
}

pub fn output() {
    let r = result();
    println!("-----\nDay 3\n-----\nPart 1: {}\nPart 2: {}\n", r.0, r.1);
}

#[test]
fn test_update_position() {
    assert_eq!(update_position('v', (1, 1)), (1, 0));
    assert_eq!(update_position('x', (1, 1)), (1, 1));
}

#[test]
fn test_result() {
    assert_eq!(result(), (2565, 2639));
}
