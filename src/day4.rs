use std::iter::repeat;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn proved_work(s: String, n: usize) -> bool {
    let zeroes = repeat('0').take(n).collect::<String>();
    s.starts_with(&zeroes)
}

pub fn part1(input: &str) -> u64 {
    /* TODO: What's wrong with this?
     * let mut result: [u8; 16];
     * md5.result(&result);
     */

    let mut md5 = Md5::new();
    let mut result = String::new();
    let mut i = 0;

    while !proved_work(result, 5) {
        i += 1;
        md5.input_str(&format!("{}{}", &input, i));
        result = md5.result_str();
        println!("{}", result);
        md5.reset();
    }

    i
}

pub fn part2(input: &str) -> u32 {
    let mut md5 = Md5::new();
    let mut result = String::new();
    let mut i = 0;

    while !proved_work(result, 6) {
        i += 1;
        md5.input_str(&format!("{}{}", &input, i));
        result = md5.result_str();
        md5.reset();
    }

    i
}

pub fn result() -> (u64, u32) {
    let input = "bgvyzdsv";

    (part1(&input), part2(&input))
}

pub fn output() {
    let r = result();
    println!("-----\nDay 4\n-----\nPart 1: {}\nPart 2: {}\n", r.0, r.1);
}

#[test]
fn test_proved_work() {
    assert!(proved_work("00000".to_string(), 5,));
    assert!(proved_work("00000x".to_string(), 5));
    assert!(!proved_work("sohustahesu".to_string(), 1));
}

#[test]
#[ignore]
fn test_result() {
    assert_eq!(result(), (254575, 1038736));
}
