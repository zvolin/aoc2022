use std::collections::VecDeque;
use std::str::FromStr;

static INPUT: &str = include_str!("../input");

struct Monkey {
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    on_true: usize,
    on_false: usize,
    inspects: usize,
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.lines().skip(1);
        let (_, items) = input.next().unwrap().split_once(": ").unwrap();
        let items = items
            .split(' ')
            .map(|item| item.strip_suffix(',').unwrap_or(item))
            .map(|item| item.trim().parse().unwrap())
            .collect();
        let (_, equation) = input
            .next()
            .unwrap()
            .split_once("= ")
            .expect("Invalid operation");
        let vars: Vec<_> = equation.split(' ').map(str::trim).collect();
        let operation: Box<dyn Fn(usize) -> usize> = match vars.as_slice() {
            ["old", "+", "old"] => Box::new(move |x| x + x),
            ["old", "*", "old"] => Box::new(move |x| x * x),
            ["old", "+", constant] | [constant, "+", "old"] => {
                let constant: usize = constant.parse().map_err(|_| "Invalid operation")?;
                Box::new(move |x| x + constant)
            }
            ["old", "*", constant] | [constant, "*", "old"] => {
                let constant: usize = constant.parse().map_err(|_| "Invalid operation")?;
                Box::new(move |x| x * constant)
            }
            _ => Err("Invalid operation")?,
        };
        let test_divisor: usize = input
            .next()
            .unwrap()
            .rsplit_once(' ')
            .unwrap()
            .1
            .parse()
            .unwrap();
        let test = Box::new(move |x| x % test_divisor == 0);
        let [on_true, on_false] = std::array::from_fn(|_| {
            input
                .next()
                .expect("Unexpected end of Monkey")
                .rsplit_once(' ')
                .expect("Invalid Test")
                .1
                .parse()
                .expect("Invalid target")
        });

        Ok(Self {
            items,
            operation,
            test,
            on_true,
            on_false,
            inspects: 0,
        })
    }
}

impl Monkey {
    // Take a turn and maybe return the (monkey, item) pair.
    fn take_turn(&mut self, part2: bool) -> Option<(usize, usize)> {
        let item = self.items.pop_front()?;
        let inspection_worry = (self.operation)(item);
        self.inspects += 1;
        let after_inspection_worry = if !part2 {
            inspection_worry / 3
        } else {
            inspection_worry
        };
        match (self.test)(after_inspection_worry) {
            true => Some((self.on_true, after_inspection_worry)),
            false => Some((self.on_false, after_inspection_worry)),
        }
    }
}

fn main() {
    let mut monkeys: Vec<Monkey> = INPUT
        .split("\n\n")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    for _ in 0..20 {
        for id in 0..monkeys.len() {
            while let Some((monkey, item)) = monkeys[id].take_turn(false) {
                monkeys[monkey].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|lhs, rhs| rhs.inspects.cmp(&lhs.inspects));
    let [ref fst, ref snd] = monkeys[..2] else { unreachable!() };
    // part1
    println!("{}", fst.inspects * snd.inspects);

    let mut monkeys: Vec<Monkey> = INPUT
        .split("\n\n")
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();

    for _ in 0..20 {
        for id in 0..monkeys.len() {
            while let Some((monkey, item)) = monkeys[id].take_turn(false) {
                monkeys[monkey].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|lhs, rhs| rhs.inspects.cmp(&lhs.inspects));
    let [ref fst, ref snd] = monkeys[..2] else { unreachable!() };
    // part1
    println!("{}", fst.inspects * snd.inspects);
}
