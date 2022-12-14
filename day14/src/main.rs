#![feature(array_windows)]

use std::{
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct V {
    x: i32,
    y: i32,
}

impl V {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // fn dist(&self, end: &V) -> f64 {
    //     (((self.x - end.x).pow(2) + (self.y - end.y).pow(2)) as f64).sqrt()
    // }
}

impl FromStr for V {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (xs, ys) = s.split_once(",").ok_or_else(|| ())?;
        Ok(Self::new(
            xs.parse().map_err(|_| ())?,
            ys.parse().map_err(|_| ())?,
        ))
    }
}

impl Add for V {
    type Output = V;

    fn add(self, rhs: Self) -> Self::Output {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&V> for V {
    type Output = V;

    fn add(self, rhs: &V) -> Self::Output {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for V {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<&V> for V {
    fn add_assign(&mut self, rhs: &V) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    map: Vec<Vec<Cell>>,
    start: V,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.map {
            let s = line
                .iter()
                .map(|c| match c {
                    Cell::Air => ' ',
                    Cell::Rock => '#',
                    Cell::Sand => 'o',
                })
                .collect::<String>();
            writeln!(f, "{s}")?;
        }
        Ok(())
    }
}

impl Map {
    fn new(width: usize, height: usize, start: V) -> Self {
        let mut map = Vec::new();
        for _ in 0..height {
            map.push(vec![Cell::Air; width]);
        }
        Self { map, start }
    }

    fn get<'a>(&'a self, v: &V) -> Option<&'a Cell> {
        if v.y < 0 || v.x < 0 {
            return None;
        }
        self.map
            .get(v.y as usize)
            .map(|l| l.get(v.x as usize))
            .flatten()
    }

    fn get_mut<'a>(&'a mut self, v: &V) -> Option<&'a mut Cell> {
        if v.y < 0 || v.x < 0 {
            return None;
        }
        self.map
            .get_mut(v.y as usize)
            .map(|l| l.get_mut(v.x as usize))
            .flatten()
    }
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    let mut min_x = 99999999;
    let mut max_x = -99999999;
    let mut min_y = 99999999;
    let mut max_y = -99999999;

    let mut lines = Vec::new();
    for l in input.lines() {
        let mut steps = Vec::new();
        for step in l.split(" -> ") {
            let v: V = step.parse().unwrap();
            min_x = min_x.min(v.x);
            max_x = max_x.max(v.x);
            min_y = min_y.min(v.y);
            max_y = max_y.max(v.y);

            steps.push(v);
        }
        lines.push(steps);
    }
    min_x = min_x.min(0);
    max_x = max_x.max(1000);
    min_y = min_y.min(0);
    max_y = max_y.max(0);

    lines.push(vec![V::new(0, max_y + 2), V::new(999, max_y + 2)]);

    max_x += 1;
    max_y += 3;
    min_x -= 1;
    min_y -= 1;

    let mut map = Map::new(
        (max_x - min_x) as usize,
        (max_y - min_y) as usize,
        V::new(500 - min_x, -min_y),
    );

    for line in lines {
        for [from, to] in line.array_windows() {
            for x in from.x.min(to.x)..=to.x.max(from.x) {
                for y in from.y.min(to.y)..=to.y.max(from.y) {
                    *map.get_mut(&V::new(x - min_x, y - min_y)).unwrap() = Cell::Rock;
                }
            }
        }
    }

    let mut amount = 0;
    'outer: loop {
        let mut sand = map.start.clone();

        loop {
            if map.get(&(sand.clone() + V::new(0, 1))) == Some(&Cell::Air) {
                sand += V::new(0, 1);
            } else if map.get(&(sand.clone() + V::new(-1, 1))) == Some(&Cell::Air) {
                sand += V::new(-1, 1);
            } else if map.get(&(sand.clone() + V::new(1, 1))) == Some(&Cell::Air) {
                sand += V::new(1, 1);
            } else if map.get(&(sand.clone() + V::new(0, 1))) == None {
                break 'outer;
            } else {
                match map.get_mut(&sand) {
                    Some(c) => {
                        amount += 1;
                        *c = Cell::Sand;
                        if sand == map.start {
                            break 'outer;
                        }
                        break;
                    }
                    None => break 'outer,
                }
            }
        }
    }

    println!("{map}\n{amount}");
}
