use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    // What is the sum of the possible games IDs that could have been played with:
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    fn filter(g: &Game) -> bool {
        g.reveals
            .iter()
            .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
    }

    let answer = INPUT
        .lines()
        .map(|line| Game::from_str(line).unwrap())
        .filter(filter)
        .map(|g| g.id)
        .sum::<usize>();

    println!("The first answer is: {answer}");
}

struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(':').unwrap();
        let id = left.strip_prefix("Game ").unwrap().parse().unwrap();

        let mut reveals = Vec::new();
        for chunk in right.split(';') {
            reveals.push(Reveal::from_str(chunk).unwrap());
        }

        Ok(Game { id, reveals })
    }
}

#[derive(Debug, Default)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Reveal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut reveal = Reveal::default();
        for chunk in s.split(',') {
            let (left, right) = chunk.trim().split_once(' ').unwrap();
            let count = left.parse().unwrap();
            match right {
                "red" => reveal.red = count,
                "green" => reveal.green = count,
                "blue" => reveal.blue = count,
                _ => return Err(()),
            }
        }
        Ok(reveal)
    }
}
