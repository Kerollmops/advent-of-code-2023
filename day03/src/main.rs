use std::str::FromStr;

use slice_group_by::StrGroupBy;

const INPUT: &str = include_str!("../example-input.txt");

fn main() -> anyhow::Result<()> {
    // let mut previous_elements = None;

    let answer = INPUT.lines().for_each(|s| {
        let e = ElementsSpans::from_str(s).unwrap();
        dbg!(e);
    });

    // println!("The first answer is: {answer}");

    Ok(())
}

#[derive(Debug)]
struct ElementsSpans(Vec<(Span, Element)>);

impl FromStr for ElementsSpans {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions = Vec::new();
        for chunk in s.linear_group_by_key(CharType::from).filter(|s| !s.starts_with('.')) {
            let element = Element::from(chunk);
            let span = Span::from_strs(s, chunk);
            positions.push((span, element));
        }
        Ok(ElementsSpans(positions))
    }
}

#[derive(Debug, PartialEq)]
enum CharType {
    Digit,
    Symbol,
    Dot,
}

impl From<char> for CharType {
    fn from(c: char) -> CharType {
        match c {
            '.' => CharType::Dot,
            '0'..='9' => CharType::Digit,
            _ => CharType::Symbol,
        }
    }
}

#[derive(Debug)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// What an horrifying C/Cpp-style hack :puke:
    fn from_strs(main: &str, sub: &str) -> Span {
        let start = main.as_ptr() as usize;
        let sub_len = sub.len();
        let sub = sub.as_ptr() as usize;
        let start = sub.checked_sub(start).unwrap();
        Span { start, end: start + sub_len }
    }
}

#[derive(Debug)]
enum Element {
    Number(u32),
    Symbol,
}

impl From<&str> for Element {
    fn from(s: &str) -> Self {
        s.parse().map(Element::Number).unwrap_or(Element::Symbol)
    }
}
