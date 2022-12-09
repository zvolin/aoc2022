use std::{collections::HashSet, iter, str::FromStr};
use Direction::*;

static INPUT: &str = include_str!("../input");

type Pos = (isize, isize);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Direction from first point to second point
    fn from_points((fstx, fsty): Pos, (sndx, sndy): Pos) -> (Option<Self>, Option<Self>) {
        let x_diff = (fstx - sndx).abs();
        let y_diff = (fsty - sndy).abs();

        if x_diff <= 1 && y_diff <= 1 {
            return (None, None);
        }

        let horizontal = if (fstx - sndx).abs() > 1 {
            if fstx > sndx {
                Some(Left)
            } else {
                Some(Right)
            }
        } else {
            None
        };
        let vertical = if (fsty - sndy).abs() > 1 {
            if fsty > sndy {
                Some(Down)
            } else {
                Some(Up)
            }
        } else {
            None
        };
        (horizontal, vertical)
    }
}

impl Direction {
    fn apply(&self, (x, y): Pos) -> Pos {
        match *self {
            Up => (x, y + 1),
            Down => (x, y - 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Step {
    direction: Direction,
    times: usize,
}

impl FromStr for Step {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some((direction, times)) = input.split_once(' ') else {
            Err("Invalid input")?
        };
        Ok(Self {
            times: times.parse().map_err(|_| "Invalid digit")?,
            direction: match direction {
                "R" => Right,
                "L" => Left,
                "U" => Up,
                "D" => Down,
                _ => Err("Invalid direction")?,
            },
        })
    }
}

fn next_tail((headx, heady): Pos, (tailx, taily): Pos) -> Pos {
    match Direction::from_points((tailx, taily), (headx, heady)) {
        (Some(horizontal), Some(vertical)) => vertical.apply(horizontal.apply((tailx, taily))),
        (Some(direction), _) | (_, Some(direction)) => match direction {
            Up => (headx, heady - 1),
            Down => (headx, heady + 1),
            Left => (headx + 1, heady),
            Right => (headx - 1, heady),
        },
        _ => (tailx, taily),
    }
}

fn print_rope(rope: &[Pos]) {
    let minx = rope.iter().map(|(x, _)| x).min().unwrap();
    let maxx = rope.iter().map(|(x, _)| x).max().unwrap();
    let miny = rope.iter().map(|(_, y)| y).min().unwrap();
    let maxy = rope.iter().map(|(_, y)| y).max().unwrap();

    for y in (miny - 4..maxy + 4).rev() {
        for x in minx - 4..maxx + 4 {
            if let Some((idx, _)) = rope.iter().enumerate().find(|(_, &p)| p == (x, y)) {
                print!("{idx}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let _input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let mut positions = HashSet::new();
    let mut head = (0, 0);
    let mut tail = head;
    positions.insert(tail);
    for line in INPUT.lines() {
        let step: Step = line.parse().unwrap();
        for _ in 0..step.times {
            head = step.direction.apply(head);
            tail = next_tail(head, tail);
            positions.insert(tail);
        }
    }

    // part 1
    println!("{}", positions.len());

    let mut positions = HashSet::new();
    let mut rope = vec![(0, 0); 10];
    positions.insert((0, 0));
    for line in INPUT.lines() {
        // print_rope(&rope);

        let step: Step = line.parse().unwrap();
        for _ in 0..step.times {
            rope[0] = step.direction.apply(rope[0]);
            for n in 1..10 {
                rope[n] = next_tail(rope[n - 1], rope[n]);
            }
            positions.insert(rope[9]);
        }
    }

    // part 2
    println!("{}", positions.len());
}
