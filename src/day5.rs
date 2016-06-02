use std::io::prelude::*;
use std::fs::File;

// Let's give this newtype pattern a shot...
struct Word(String);

impl Word {
    fn is_nice_part_1(&self) -> bool {
        !self.has_forbidden() &&
        self.has_pair(0) &&
        self.has_three_vowels()
    }

    fn is_nice_part_2(&self) -> bool {
        self.has_two_non_overlapping_pairs() && self.has_pair(1)
    }

    fn has_forbidden(&self) -> bool {
        let forbidden = ["ab", "cd", "pq", "xy"];
        forbidden.iter().any(|&x| self.0.contains(x))
    }

    fn has_pair(&self, seperator: usize) -> bool {
        let bytes = self.0.as_bytes();
        let offset = seperator + 1;

        // If the word is not as long as separator + 1, a pair is not possible
        if bytes.len() <= offset { return false }

        for i in 0..bytes.len()-offset {
            if bytes[i] == bytes[i+offset] { return true }
        }
        false
    }

    fn has_three_vowels(&self) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut count = 0;
        let mut chars = self.0.chars();

        while count < 3 {
            if let Some(c) = chars.next() {
                if vowels.contains(&c) { count += 1; }
            }
            else { return false;  }
        }
        true
    }

    fn has_two_non_overlapping_pairs(&self) -> bool {
        let bytes = self.0.as_bytes();
        let len = bytes.len();
        if len < 4 { return false }

        for i in 0..len-2 {
            let pair = (bytes[i], bytes[i+1]);
            let rem = &bytes[i+2..];

            for j in 0..rem.len()-1 {
                if pair == (rem[j], rem[j+1]) { return true }
            }
        }
        false
    }
}

fn part1(words: &Vec<Word>) -> usize {
    words.iter().filter(|&word| word.is_nice_part_1()).count()
}

fn part2(words: &Vec<Word>) -> usize {
    words.iter().filter(|&word| word.is_nice_part_2()).count()
}

pub fn result() -> (usize, usize) {
    let mut f = File::open("data/day5").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let words = input.split('\n')
        .map(|l| Word(l.to_string()))
        .collect();

    (part1(&words), part2(&words))
}

pub fn output() {
    let r = result();
    println!("-----\nDay 5\n-----\nPart 1: {}\nPart 2: {}\n", r.0, r.1);
}

#[test]
fn test_has_forbidden() {
    assert!(Word("ab".to_string()).has_forbidden());
    assert!(Word("abcdpqxy".to_string()).has_forbidden());
    assert!(!Word("xxx".to_string()).has_forbidden());
}

#[test]
fn test_has_pair() {
    assert!(Word("aa".to_string()).has_pair(0));
    assert!(Word("aaa".to_string()).has_pair(0));
    assert!(!Word("ab".to_string()).has_pair(0));
    assert!(Word("xyx".to_string()).has_pair(1));
    assert!(Word("aaa".to_string()).has_pair(1));
    assert!(!Word("a".to_string()).has_pair(1));
    assert!(!Word("a".to_string()).has_pair(0));
}

#[test]
fn test_has_three_vowels() {
    assert!(Word("aeo".to_string()).has_three_vowels());
    assert!(Word("aaa".to_string()).has_three_vowels());
    assert!(!Word("aa".to_string()).has_three_vowels());
    assert!(!Word("xoxo".to_string()).has_three_vowels());
}

#[test]
fn test_has_two_non_overlapping_pairs() {
    assert!(Word("xyxy".to_string()).has_two_non_overlapping_pairs());
    assert!(Word("xyxxy".to_string()).has_two_non_overlapping_pairs());
    assert!(!Word("aaa".to_string()).has_two_non_overlapping_pairs());
    assert!(!Word("abcd".to_string()).has_two_non_overlapping_pairs());
    assert!(!Word("a".to_string()).has_two_non_overlapping_pairs());
}

#[test]
fn test_result() {
    assert_eq!(result(), (255, 55));
}
