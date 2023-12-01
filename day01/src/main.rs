use std::str::from_utf8;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let answer = INPUT
        .lines()
        .map(|line| {
            let first = line.chars().find(char::is_ascii_digit).unwrap();
            let last = line.chars().rev().find(char::is_ascii_digit).unwrap();
            let v = &[first as u8, last as u8];
            let s = from_utf8(v).unwrap();
            s.parse::<usize>().unwrap()
        })
        .sum::<usize>();

    println!("The first answer is: {answer}");

    let answer = INPUT
        .lines()
        .map(|line| {
            let first = find_first_digit(line) as usize;
            let last = find_last_digit(line) as usize;
            first * 10 + last
        })
        .sum::<usize>();

    println!("The second answer is: {answer}");
}

const DIGITS: &[(&str, u8)] = &[
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_first_digit(s: &str) -> u8 {
    DIGITS
        .iter()
        .min_by_key(|(word, digit)| s.find(word).map_or((usize::MAX, u8::MAX), |p| (p, *digit)))
        .map(|(_, d)| *d)
        .unwrap()
}

fn find_last_digit(s: &str) -> u8 {
    DIGITS
        .iter()
        .max_by_key(|(word, digit)| s.rfind(word).map_or((usize::MIN, u8::MIN), |p| (p, *digit)))
        .map(|(_, d)| *d)
        .unwrap()
}
