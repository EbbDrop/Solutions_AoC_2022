use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone)]
struct Display2D(Vec<String>);

impl Display2D {
    fn new(w: usize, h: usize, start_char: char) -> Self {
        let mut d = Vec::with_capacity(h);

        for _ in 0..h {
            d.push(std::iter::repeat(start_char).take(w).collect())
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
}

impl Display for Display2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.0 {
            writeln!(f, "{}", l)?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct V {
    x: i32,
    y: i32,
}

impl V {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    let mut elfs = HashSet::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '#' {
                continue;
            }
            let v = V::new(x as i32, y as i32);
            elfs.insert(v);
        }
    }

    let mut search_dir = VecDeque::from([V::new(0, -1), V::new(0, 1), V::new(-1, 0), V::new(1, 0)]);

    let mut steps = 0;
    loop {
        // This maps new pos to old pos
        let mut provisions = HashMap::new();

        'next_elf: for elf in &elfs {
            'has_to_move: loop {
                for x_diff in -1..=1 {
                    for y_diff in -1..=1 {
                        if x_diff == 0 && y_diff == 0 {
                            continue;
                        }
                        if elfs.contains(&V::new(elf.x + x_diff, elf.y + y_diff)) {
                            break 'has_to_move;
                        }
                    }
                }
                continue 'next_elf;
            }

            'next_dir: for dir in &search_dir {
                for other in -1..=1 {
                    let v = if dir.x == 0 {
                        V::new(other, dir.y) + elf
                    } else {
                        V::new(dir.x, other) + elf
                    };

                    if elfs.contains(&v) {
                        continue 'next_dir;
                    }
                }
                let new_pos = elf.clone() + dir;
                if provisions.contains_key(&new_pos) {
                    *provisions.get_mut(&new_pos).unwrap() = None;
                } else {
                    provisions.insert(new_pos, Some(elf.clone()));
                }
                break;
            }
        }
        if provisions.is_empty() {
            break;
        }

        for (_, from) in &provisions {
            if let &Some(ref from) = from {
                elfs.remove(from);
            }
        }
        for (to, from) in provisions {
            if from.is_some() {
                elfs.insert(to);
            }
        }

        search_dir.rotate_left(1);

        // let mut min = elfs.iter().next().unwrap().clone();
        // let mut max = min.clone();
        // for elf in &elfs {
        //     min.x = min.x.min(elf.x);
        //     min.y = min.y.min(elf.y);

        //     max.x = max.x.max(elf.x);
        //     max.y = max.y.max(elf.y);
        // }

        // let mut d = Display2D::new(
        //     (max.x - min.x + 1) as usize,
        //     (max.y - min.y + 1) as usize,
        //     '.',
        // );
        // for elf in &elfs {
        //     d.draw(elf.x - min.x, elf.y - min.y, '#');
        // }
        // println!("{}", d);

        // dbg!(((max.x - min.x + 1) * (max.y - min.y + 1)) - elfs.len() as i32);
        steps += 1;
    }
    dbg!(steps + 1);
}
