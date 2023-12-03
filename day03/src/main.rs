use std::collections::hash_map::Entry;
use std::collections::HashMap;

use ndarray::Array2;
use slice_group_by::{GroupBy, StrGroupBy};

const INPUT: &str = include_str!("../input.txt");

fn main() -> anyhow::Result<()> {
    let rows = INPUT.lines().count();
    let columns = INPUT.lines().next().unwrap().len();

    // We generate the array conresponding to the input
    let mut array = Array2::<char>::default((rows, columns));
    for (r, line) in INPUT.lines().enumerate() {
        for (c, x) in line.chars().enumerate() {
            array[[r, c]] = x;
        }
    }

    // We modify the array to alterate every number that must be considered.
    for (r, line) in INPUT.lines().enumerate() {
        for (c, x) in line.chars().enumerate() {
            if let CharType::Symbol(_) = CharType::from(x) {
                touch_square_around(&mut array, [r, c], '.');
            }
        }
    }

    // Then we iterate on both strings at the same time, the original one and the modified one.
    // Eevrytime the extracted number string is different between both chunks
    // it means that it was near a symbol and we need to take it.
    let touched_array = ndarray_into_string(&array);
    let mut numbers = Vec::new();
    for (chunk, touched_chunk) in INPUT.lines().zip(touched_array.lines()) {
        let iter = parallel_iter(chunk, touched_chunk)
            .filter_map(|(a, b)| (a != b).then_some(a))
            .filter(|ct| CharType::from(ct.chars().next().unwrap()).is_number())
            .map(|s| s.parse::<usize>().unwrap());
        numbers.extend(iter);
    }

    let answer: usize = numbers.iter().sum();
    println!("The first answer is: {answer}");

    // ---- PART 2 ----

    let rows = INPUT.lines().count();
    let columns = INPUT.lines().next().unwrap().len();

    // We generate the array conresponding to the input
    let mut array = Array2::<char>::default((rows, columns));
    for (r, line) in INPUT.lines().enumerate() {
        for (c, x) in line.chars().enumerate() {
            array[[r, c]] = x;
        }
    }

    // We modify the array to alterate every number that must be considered.
    let mut id = 'A'..;
    for (r, line) in INPUT.lines().enumerate() {
        for (c, x) in line.chars().enumerate() {
            if let CharType::Symbol('*') = CharType::from(x) {
                if symbol_touches_exactly_two_digits(&array, [r, c]) {
                    let id = id.next().unwrap();
                    touch_square_around(&mut array, [r, c], id);
                }
            }
        }
    }

    // Then we iterate on both strings at the same time, the original one and the modified one.
    // Eevrytime the extracted number string is different between both chunks
    // it means that it was near a symbol and we need to take it.
    let touched_array = ndarray_into_string(&array);

    let mut numbers = HashMap::new();
    for (chunk, touched_chunk) in INPUT.lines().zip(touched_array.lines()) {
        let iter = parallel_iter(chunk, touched_chunk)
            .filter(|(original, _)| CharType::from(original.chars().next().unwrap()).is_number())
            .filter_map(|(original, touched)| char_diff(original, touched).map(|c| (original, c)))
            .map(|(original, c)| (c, original.parse::<usize>().unwrap()));
        for (c, n) in iter {
            match numbers.entry(c) {
                Entry::Occupied(mut entry) => *entry.get_mut() *= n,
                Entry::Vacant(entry) => {
                    entry.insert(n);
                }
            }
        }
    }

    let answer: usize = numbers.values().copied().sum();
    println!("The second answer is: {answer}");

    Ok(())
}

fn char_diff(original: &str, touched: &str) -> Option<char> {
    original.chars().zip(touched.chars()).find_map(|(o, t)| (o != t).then_some(t))
}

fn parallel_iter<'a>(
    input: &'a str,
    mut copy: &'a str,
) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
    input.linear_group_by_key(CharType::from).map(move |chunk| {
        let (i, c) = copy.char_indices().take(chunk.len()).last().unwrap();
        let (left, right) = copy.split_at(i + c.len_utf8());
        copy = right;
        (chunk, left)
    })
}

fn ndarray_into_string(array: &Array2<char>) -> String {
    let mut touched_input = String::new();
    for row in array.rows() {
        for x in row {
            touched_input.push(*x);
        }
        touched_input.push('\n');
    }
    touched_input
}

fn symbol_touches_exactly_two_digits(array: &Array2<char>, [r, c]: [usize; 2]) -> bool {
    #[rustfmt::skip]
    let ops: [[isize; 2]; 9] = [
        [-1,-1], [-1, 0], [-1, 1],
        [ 0,-1], [ 0, 0], [ 0, 1],
        [ 1,-1], [ 1, 0], [ 1, 1],
    ];

    let is_digit = |[ro, co]: &[isize; 2]| {
        let r: usize = match (r as isize + ro).try_into().ok() {
            Some(r) => r,
            None => return false,
        };
        let c: usize = match (c as isize + co).try_into().ok() {
            Some(c) => c,
            None => return false,
        };
        let value = array.get([r, c]).copied().map(CharType::from);
        value == Some(CharType::Digit)
    };

    let mut matches: Vec<_> = ops.iter().map(is_digit).collect();
    // inserts falses to split lanes
    matches.insert(3, false);
    matches.insert(7, false);

    let groups = matches.linear_group_by_key(|b| *b).filter(|s| s[0]).count();

    groups == 2
}

fn touch_square_around(array: &mut Array2<char>, [r, c]: [usize; 2], v: char) {
    #[rustfmt::skip]
    let ops: [[isize; 2]; 9] = [
        [-1,-1], [-1, 0], [-1, 1],
        [ 0,-1], [ 0, 0], [ 0, 1],
        [ 1,-1], [ 1, 0], [ 1, 1],
    ];
    ops.iter()
        .flat_map(|[ro, co]| {
            let r: usize = (r as isize + ro).try_into().ok()?;
            let c: usize = (c as isize + co).try_into().ok()?;
            Some([r, c])
        })
        .for_each(|[r, c]| {
            if let Some(x) = array.get_mut([r, c]) {
                *x = v;
            }
        });
}

#[derive(Debug, PartialEq)]
enum CharType {
    Digit,
    Symbol(char),
    Dot,
}

impl CharType {
    fn is_number(&self) -> bool {
        matches!(self, CharType::Digit)
    }
}

impl From<char> for CharType {
    fn from(c: char) -> CharType {
        match c {
            '.' => CharType::Dot,
            '0'..='9' => CharType::Digit,
            c => CharType::Symbol(c),
        }
    }
}
