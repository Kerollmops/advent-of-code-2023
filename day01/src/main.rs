use aho_corasick::AhoCorasick;
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
            let (first, last) = find_first_and_last_digit(line);
            first * 10 + last
        })
        .sum::<usize>();

    println!("The second answer is: {answer}");
}

const DIGIT_WORDS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

const DIGITS: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

fn find_first_and_last_digit(line: &str) -> (usize, usize) {
    let ac = AhoCorasick::builder()
        .ascii_case_insensitive(false)
        .build(DIGIT_WORDS)
        .unwrap();

    // FUCK IT! Don't forget that it must be overlapping!!!
    let mut iter = ac.find_overlapping_iter(line);
    let first_id = iter.next().unwrap().pattern().as_usize();
    let last_id = iter.last().map_or(first_id, |m| m.pattern().as_usize());

    (DIGITS[first_id], DIGITS[last_id])
}
