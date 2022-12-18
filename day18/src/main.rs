use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct V {
    x: i64,
    y: i64,
    z: i64,
}

impl V {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl Add for V {
    type Output = V;

    fn add(self, rhs: Self) -> Self::Output {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&V> for V {
    type Output = V;

    fn add(self, rhs: &V) -> Self::Output {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&V> for &V {
    type Output = V;

    fn add(self, rhs: &V) -> Self::Output {
        V {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for V {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&V> for V {
    fn add_assign(&mut self, rhs: &V) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Air,
    Lava,
    Steam,
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    map: Vec<Vec<Vec<Cell>>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for plane in &self.map {
            for line in plane {
                let s = line
                    .iter()
                    .map(|c| match c {
                        Cell::Air => ' ',
                        Cell::Lava => '#',
                        Cell::Steam => 'o',
                    })
                    .collect::<String>();
                writeln!(f, "{s}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        let mut map = Vec::new();
        for _ in 0..height {
            let mut plane = Vec::new();
            for _ in 0..depth {
                plane.push(vec![Cell::Air; width]);
            }
            map.push(plane);
        }
        Self { map }
    }

    fn get<'a>(&'a self, v: &V) -> Option<&'a Cell> {
        if v.y < 0 || v.x < 0 {
            return None;
        }
        self.map
            .get(v.y as usize)
            .and_then(|l| l.get(v.x as usize))
            .and_then(|l| l.get(v.z as usize))
    }

    fn get_mut<'a>(&'a mut self, v: &V) -> Option<&'a mut Cell> {
        if v.y < 0 || v.x < 0 {
            return None;
        }
        self.map
            .get_mut(v.y as usize)
            .and_then(|l| l.get_mut(v.x as usize))
            .and_then(|l| l.get_mut(v.z as usize))
    }

    fn flood(&mut self, start: V) {
        if matches!(self.get(&start), None | Some(&Cell::Steam | &Cell::Lava)) {
            return;
        }
        *self.get_mut(&start).unwrap() = Cell::Steam;

        for side in &[
            V::new(-1, 0, 0),
            V::new(1, 0, 0),
            V::new(0, -1, 0),
            V::new(0, 1, 0),
            V::new(0, 0, -1),
            V::new(0, 0, 1),
        ] {
            self.flood(&start + side);
        }
    }
}

fn main() {
    // let intput = include_str!("./example.txt");
    let intput = include_str!("./input.txt");

    let size = 22;

    let mut map = Map::new(size, size, size);

    for l in intput.lines() {
        let mut values = l.split(",");

        let p = V {
            x: values.next().unwrap().parse().unwrap(),
            y: values.next().unwrap().parse().unwrap(),
            z: values.next().unwrap().parse().unwrap(),
        };

        *(map.get_mut(&(&p + &V::new(1, 1, 1))).unwrap()) = Cell::Lava;
    }

    map.flood(V::new(0, 0, 0));

    let mut amount = 0;

    println!("{}", &map);

    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                let p = V::new(x as i64, y as i64, z as i64);
                if map.get(&p) != Some(&Cell::Lava) {
                    continue;
                }
                for side in &[
                    V::new(-1, 0, 0),
                    V::new(1, 0, 0),
                    V::new(0, -1, 0),
                    V::new(0, 1, 0),
                    V::new(0, 0, -1),
                    V::new(0, 0, 1),
                ] {
                    if map.get(&(&p + side)) == Some(&Cell::Steam) {
                        amount += 1;
                    }
                }
            }
        }
    }

    dbg!(amount);
}
