use std::{fmt::Display, iter::repeat, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn cycle_length(&self) -> i64 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Self::Noop)
        } else {
            let (_, n) = s.split_once(" ").unwrap();
            Ok(Self::Addx(n.parse().unwrap()))
        }
    }
}

#[derive(Debug, Clone)]
struct Display2D(Vec<String>);

impl Display2D {
    fn new(w: usize, h: usize) -> Self {
        let mut d = Vec::with_capacity(h);

        for _ in 0..h {
            d.push(repeat(' ').take(w).collect())
        }

        Display2D(d)
    }

    fn draw(&mut self, x: i32, y: i32, c: char) {
        if y >= 0 && y < self.0.len() as i32 {
            let l = &mut self.0[y as usize];
            if x >= 0 && x < l.len() as i32 {
                l.replace_range(x as usize..x as usize + 1, &c.to_string());
            }
        }
    }

    fn clear(&mut self) {
        for l in &mut self.0 {
            let len = l.len();
            l.clear();
            l.push_str(&repeat('.').take(len).collect::<String>());
        }
    }
}

impl Display for Display2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.0 {
            writeln!(f, "{}", l)?
        }
        Ok(())
    }
}

fn main() {
    // let lines = include_str!("./example.txt").lines();
    let lines = include_str!("./input.txt").lines();

    let mut reg_x = 1;
    let mut cycle = 1;

    let mut screen = Display2D::new(41, 6);

    let insts = lines.map(|l| l.parse::<Instruction>().unwrap());
    for inst in insts {
        let cycle_afther_inst = cycle + inst.cycle_length();
        for x in cycle..cycle_afther_inst {
            let y = (x - 1) / 40;
            let x = (x - 1) % 40;

            if x - 1 == reg_x || x == reg_x || x + 1 == reg_x {
                screen.draw(x as i32, y as i32, '#')
            }
        }

        match inst {
            Instruction::Noop => {}
            Instruction::Addx(n) => reg_x += n,
        }

        cycle = cycle_afther_inst;

        println!("{}", screen);
    }
}
