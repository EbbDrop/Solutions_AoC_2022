use std::{
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

    fn dist(&self, end: &V) -> f64 {
        (((self.x - end.x).pow(2) + (self.y - end.y).pow(2)) as f64).sqrt()
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
struct Cell {
    pos: V,
    height: i32,
    visited: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    map: Vec<Vec<Cell>>,
    start: V,
    end: V,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start_v = None;
        let mut end_v = None;
        let mut map = Vec::new();
        for (y, l) in s.lines().enumerate() {
            let y = y as i32;
            let mut cell_line = Vec::new();

            for (x, mut c) in l.chars().enumerate() {
                if c == 'S' {
                    c = 'a';
                    start_v = Some(V::new(x as i32, y));
                }
                if c == 'E' {
                    c = 'z';
                    end_v = Some(V::new(x as i32, y));
                }
                cell_line.push(Cell {
                    pos: V::new(x as i32, y),
                    height: c as i32 - 'a' as i32,
                    visited: None,
                })
            }
            map.push(cell_line);
        }

        Ok(Self {
            map,
            start: start_v.unwrap(),
            end: end_v.unwrap(),
        })
    }
}

impl Map {
    fn neighbors(&self, v: &V) -> Vec<V> {
        let height = self.get(v).unwrap().height;
        [V::new(-1, 0), V::new(1, 0), V::new(0, -1), V::new(0, 1)]
            .into_iter()
            .filter_map(|diff| -> Option<V> {
                let new_v = v.clone() + diff;
                let new_height = self.get(&new_v)?.height;
                if height + 1 >= new_height {
                    return Some(new_v);
                }
                None
            })
            .collect()
    }
    fn neighbors_inv(&self, v: &V) -> Vec<V> {
        let height = self.get(v).unwrap().height;
        [V::new(-1, 0), V::new(1, 0), V::new(0, -1), V::new(0, 1)]
            .into_iter()
            .filter_map(|diff| -> Option<V> {
                let new_v = v.clone() + diff;
                let new_height = self.get(&new_v)?.height;
                if height - 1 <= new_height {
                    return Some(new_v);
                }
                None
            })
            .collect()
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

fn do_a_star_from(start: &V, map: &mut Map) {
    let mut to_search: Vec<V> = Vec::new();
    to_search.push(start.clone());
    let start_cell = map.get_mut(start).unwrap();

    if start_cell.visited.is_some() {
        return;
    }
    start_cell.visited = Some(0);

    let mut done = false;
    while !done {
        let Some(current_v) = to_search.pop() else {
            break;
        };
        let current_dist_from_start = map.get(&current_v).unwrap().visited.unwrap();

        for neighbor in map.neighbors_inv(&current_v) {
            let cell = map.get_mut(&neighbor).unwrap();
            match cell.visited {
                Some(from_start) => {
                    if from_start > current_dist_from_start + 1 {
                        cell.visited = Some(current_dist_from_start + 1);
                        to_search.push(cell.pos.clone());
                    }
                }
                None => {
                    cell.visited = Some(current_dist_from_start + 1);
                    to_search.push(cell.pos.clone());
                }
            }
        }
    }
}

fn main() {
    // let mut map: Map = include_str!("./example.txt").parse().unwrap();
    let mut map: Map = include_str!("./input.txt").parse().unwrap();

    let end = map.end.clone();
    do_a_star_from(&end, &mut map);

    for l in &map.map {
        let s = l
            .iter()
            .map(|c| match c.visited {
                Some(n) => {
                    if n < 10 {
                        n.to_string().chars().next().unwrap()
                    } else {
                        'H'
                    }
                }
                None => 'N',
            })
            .collect::<String>();
        println!("{s}");
    }

    let height = map.map.len();
    let mut lowest = 999999999;
    for y in 0..height {
        for x in 0..map.map[y].len() {
            let cell = map.get(&V::new(x as i32, y as i32)).unwrap();
            if cell.height != 0 {
                continue;
            }
            match cell.visited {
                Some(n) => {
                    if n < lowest {
                        lowest = n;
                    }
                    dbg!(n);
                }
                None => {}
            }
        }
    }
    dbg!(&lowest);

    // let mut curr = map.end.clone();
    // let mut steps = 0;
    // while curr != map.start {
    //     let next_curr = map
    //         .neighbors_inv(&curr)
    //         .into_iter()
    //         .filter_map(|v| {
    //             let cell = map.get(&v).unwrap().visited?;
    //             Some((v, cell.0 + cell.1))
    //         })
    //         .min_by(|a, b| a.1.total_cmp(&b.1))
    //         .unwrap()
    //         .0;
    //     map.get_mut(&curr).unwrap().visited = None;
    //     curr = next_curr;
    //     steps += 1;
    // }
    // dbg!(steps);
}
