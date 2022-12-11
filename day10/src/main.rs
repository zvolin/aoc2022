use std::{iter, str::FromStr};

static INPUT: &str = include_str!("../input");

#[derive(Clone, Copy)]
enum OpCode {
    Noop,
    Addx(isize),
}

impl FromStr for OpCode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        match parts.next() {
            Some("noop") => Ok(Self::Noop),
            Some("addx") => Ok(Self::Addx(
                parts
                    .next()
                    .ok_or("Addx requires argument")?
                    .parse()
                    .map_err(|_| "Invalid Addx argument")?,
            )),
            _ => Err("Unsupported instruction"),
        }
    }
}

struct Cpu {
    pc: usize,
    x: isize,
    clock: usize,
    timer: Option<usize>,
    program: Vec<OpCode>,
}

impl Cpu {
    fn new(program: Vec<OpCode>) -> Self {
        Self {
            pc: 0,
            x: 1,
            clock: 0,
            timer: None,
            program,
        }
    }

    fn tick(&mut self) {
        self.clock += 1;
        self.timer = self.timer.map(|tim| tim - 1);

        if let Some(0) = self.timer {
            if let OpCode::Addx(value) = self.program[self.pc] {
                self.x += value;
            }
            self.timer = None;
            self.pc += 1;
        }

        if self.timer.is_none() {
            match self.program[self.pc] {
                OpCode::Noop => self.timer = Some(1),
                OpCode::Addx(_) => self.timer = Some(2),
            }
        }
    }
}

const CRT_W: usize = 40;
const CRT_H: usize = 6;

struct Crt([bool; CRT_W * CRT_H]);

impl Crt {
    fn new() -> Self {
        Self([false; CRT_W * CRT_H])
    }

    fn draw(&mut self, sprite_offset: isize, cycle: usize) {
        let pos = cycle - 1;
        let px = (pos % CRT_W) as isize;
        if (sprite_offset - 1..=sprite_offset + 1).contains(&px) {
            self.0[pos] = true;
        }
    }

    fn show(&self) {
        println!("{:-<40}", "");
        for (n, &px) in self.0.iter().enumerate() {
            if px {
                print!("#");
            } else {
                print!(".");
            }

            if (n + 1) % CRT_W == 0 {
                println!();
            }
        }
        println!("{:-<40}", "");
    }
}

fn main() {
    let program: Vec<_> = INPUT
        .lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .collect();
    let mut cpu = Cpu::new(program.clone());

    let res = iter::once(20)
        .chain(iter::repeat(40).take(5))
        .map(|n| {
            for _ in 0..n {
                cpu.tick();
            }
            cpu.x * cpu.clock as isize
        })
        .sum::<isize>();
    // part 1
    println!("{}", res);

    let mut cpu = Cpu::new(program);
    let mut crt = Crt::new();
    for _ in 0..CRT_W * CRT_H {
        cpu.tick();
        crt.draw(cpu.x, cpu.clock);
    }
    crt.show()
}

static INPUT2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
