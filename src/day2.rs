use std::cmp;
use std::io::prelude::*;
use std::fs::File;

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(lwh: Vec<u32>) -> Present {
        Present { length: lwh[0], width: lwh[1], height: lwh[2] }
    }

    fn surface_area(&self) -> u32 {
        2 * (self.length*self.width + self.width*self.height + self.height*self.length)
    }

    fn smallest_side(&self) -> (u32, u32) {
        // TODO: this is pretty ugly
        let d1 = cmp::min(self.length, self.width);
        let d2 = cmp::min(self.height, cmp::max(self.length, self.width));
        (d1, d2)
    }

    fn wrapping_paper(&self) -> u32 {
        let smallest_side = self.smallest_side();
        self.surface_area() + area(smallest_side.0, smallest_side.1)
    }
}

pub fn get_dimensions(s: &str) -> Vec<u32> {
    s.split('x').map(|x| x.parse::<u32>().unwrap_or(0)).collect()
}

pub fn area(width: u32, height: u32) -> u32 {
    width * height
}

pub fn perimeter(width: u32, height: u32) -> u32 {
    2 * (width + height)
}

fn part1(presents: &Vec<Present>) -> u32 {
    presents.iter().fold(0, |paper, p| paper + p.wrapping_paper())
}

#[allow(unused_variables)]
fn part2(presents: &Vec<Present>) -> u32 {
    42
}

pub fn result() -> (u32, u32) {
    let mut f = File::open("data/day2").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.split('\n').collect();
    let presents = lines
        .iter()
        .map(|l| Present::new(get_dimensions(l)))
        .collect();

    (part1(&presents), part2(&presents))
}

pub fn output() {
    let r = result();
    println!("-----\nDay 2\n-----\nPart 1: {}\nPart 2: {}\n", r.0, r.1);
}

#[test]
pub fn test_get_dimensions() {
    assert_eq!(get_dimensions("1x2x3"), vec![1, 2, 3]);
}

#[test]
pub fn test_area() {
    assert_eq!(area(10, 10), 100);
}

#[test]
pub fn test_perimeter() {
    assert_eq!(perimeter(10, 10), 40);
}

#[test]
pub fn test_smallest_side() {
    let p = Present { length: 1, width: 2, height: 3 };
    assert_eq!(p.smallest_side(), (1, 2));
}

#[test]
pub fn test_surface_area() {
    let p = Present { length: 1, width: 2, height: 3 };
    assert_eq!(p.surface_area(), 22);
}

#[test]
pub fn test_result() {
    assert_eq!(result(), (1588178, 42));
}
