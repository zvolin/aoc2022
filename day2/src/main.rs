use std::str::FromStr;

static INPUT: &str = include_str!("../input");

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => unreachable!(),
        })
    }
}

impl Shape {
    fn score(&self) -> usize {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

fn round(opponent: Shape, you: Shape) -> usize {
    match (opponent, you) {
        (Shape::Rock, shape @ Shape::Rock) => shape.score() + 3,
        (Shape::Rock, shape @ Shape::Paper) => shape.score() + 6,
        (Shape::Rock, shape @ Shape::Scissors) => shape.score(),
        (Shape::Paper, shape @ Shape::Rock) => shape.score(),
        (Shape::Paper, shape @ Shape::Paper) => shape.score() + 3,
        (Shape::Paper, shape @ Shape::Scissors) => shape.score() + 6,
        (Shape::Scissors, shape @ Shape::Rock) => shape.score() + 6,
        (Shape::Scissors, shape @ Shape::Paper) => shape.score(),
        (Shape::Scissors, shape @ Shape::Scissors) => shape.score() + 3,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Loose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "X" => Outcome::Loose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => unreachable!(),
        })
    }
}

impl Shape {
    fn choice(&self, outcome: Outcome) -> Shape {
        match (*self, outcome) {
            (Shape::Rock, Outcome::Loose) => Shape::Scissors,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Loose) => Shape::Rock,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Loose) => Shape::Paper,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
            (_, Outcome::Draw) => *self,
        }
    }
}

fn main() {
    // first part
    let score = INPUT
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(opponent, you)| {
            (
                Shape::from_str(opponent).unwrap(),
                Shape::from_str(you).unwrap(),
            )
        })
        .map(|(opponent, you)| round(opponent, you))
        .sum::<usize>();
    println!("{score}");

    // second part
    let score = INPUT
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(opponent, outcome)| {
            (
                Shape::from_str(opponent).unwrap(),
                Outcome::from_str(outcome).unwrap(),
            )
        })
        .map(|(opponent, outcome)| round(opponent, opponent.choice(outcome)))
        .sum::<usize>();
    println!("{score}");
}
