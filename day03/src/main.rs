use ndarray::Array2;
use slice_group_by::StrGroupBy;

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
            if CharType::from(x) == CharType::Symbol {
                touch_square_around(&mut array, [r, c]);
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

    Ok(())
}

fn parallel_iter<'a>(
    input: &'a str,
    mut copy: &'a str,
) -> impl Iterator<Item = (&'a str, &'a str)> + 'a {
    input.linear_group_by_key(CharType::from).map(move |chunk| {
        let (left, right) = copy.split_at(chunk.len());
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

fn touch_square_around(array: &mut Array2<char>, [r, c]: [usize; 2]) {
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
                *x = '.';
            }
        });
}

#[derive(Debug, PartialEq)]
enum CharType {
    Digit,
    Symbol,
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
            _ => CharType::Symbol,
        }
    }
}
