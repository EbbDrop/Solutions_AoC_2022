use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    ops::{Add, AddAssign},
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
        steps += 1;
        // This maps new pos to old pos
        let mut provisions = HashMap::new();

        'next_elf: for elf in &elfs {
            'has_to_move: {
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

                match provisions.entry(new_pos) {
                    Entry::Occupied(mut e) => {
                        e.insert(None);
                    }
                    Entry::Vacant(e) => {
                        e.insert(Some(elf.clone()));
                    }
                }
                break 'next_dir;
            }
        }
        if provisions.is_empty() {
            break;
        }

        for from in provisions.values() {
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
    }
    dbg!(steps);
}
