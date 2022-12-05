use std::str::FromStr;

static INPUT: &str = include_str!("../input");

type Stacks = Vec<Vec<char>>;

fn parse_stacks(cargo: &str) -> Stacks {
    let lines: Vec<&str> = cargo.lines().collect();
    let (&idxs, crates) = lines.split_last().unwrap();
    let length = cargo.find('\n').unwrap();

    (0..length).fold(vec![], |mut stacks, column| {
        if idxs
            .chars()
            .nth(column)
            .expect("index out of bounds")
            .is_numeric()
        {
            stacks.push(
                crates
                    .iter()
                    .rev()
                    .filter_map(|line| {
                        let krate = line.chars().nth(column).expect("index out of bounds");
                        if krate.is_ascii_uppercase() {
                            Some(krate)
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
        }
        stacks
    })
}

struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Vec<_> = s
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .filter_map(|num| num.parse().ok())
            .collect();
        if let &[amount, from, to] = input.as_slice() {
            Ok(Self {
                from: from - 1,
                to: to - 1,
                amount,
            })
        } else {
            Err("Invalid input")
        }
    }
}

fn execute_instruction_crate_mover_9000(stacks: &mut Stacks, instruction: Instruction) {
    for _ in 0..instruction.amount {
        let krate = stacks[instruction.from].pop().expect("Invalid instruction");
        stacks[instruction.to].push(krate);
    }
}

fn execute_instruction_crate_mover_9001(stacks: &mut Stacks, instruction: Instruction) {
    let stack_len = stacks[instruction.from].len();
    let mut crates: Vec<_> = stacks[instruction.from]
        .drain(stack_len - instruction.amount..)
        .collect();
    stacks[instruction.to].append(&mut crates);
}

fn main() {
    let (cargo, instructions) = INPUT.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(cargo);

    for input in instructions.lines() {
        let instruction = Instruction::from_str(input).unwrap();
        execute_instruction_crate_mover_9000(&mut stacks, instruction);
    }

    // part 1
    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    println!();

    let mut stacks = parse_stacks(cargo);

    for input in instructions.lines() {
        let instruction = Instruction::from_str(input).unwrap();
        execute_instruction_crate_mover_9001(&mut stacks, instruction);
    }

    // part 2
    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    println!();
}
