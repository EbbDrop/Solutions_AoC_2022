use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, AddAssign},
};

use num::integer::lcm;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl Add for &V {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn to_v(&self) -> V {
        match self {
            Dir::Up => V::new(0, -1),
            Dir::Right => V::new(1, 0),
            Dir::Down => V::new(0, 1),
            Dir::Left => V::new(-1, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    pos: V,
    dir: Dir,
}

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Empty,
    Filled,
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    map: Vec<Vec<Cell>>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        let mut map = Vec::new();
        map.push(vec![Cell::Filled; width]);
        for _ in 0..height - 2 {
            let mut line = vec![Cell::Empty; width];
            line[0] = Cell::Filled;
            *line.last_mut().unwrap() = Cell::Filled;
            map.push(line);
        }
        map.push(vec![Cell::Filled; width]);
        map[0][1] = Cell::Empty;
        *map.last_mut()
            .unwrap()
            .iter_mut()
            .rev()
            .skip(1)
            .next()
            .unwrap() = Cell::Empty;
        Self { map }
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

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in &self.map {
            for cell in l {
                let char = match cell {
                    Cell::Empty => '.',
                    Cell::Filled => '#',
                };
                write!(f, "{char}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    pos: V,
    time: usize,
    dist: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.time + other.dist)
            .cmp(&(self.time + self.dist))
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// fn do_a_start(start: V, start_time: usize, end: V, maps: &Vec<Map>) -> Option<usize> {
//     let start_state = State {
//         pos: start.clone(),
//         time: start_time,
//         dist: start.dist(&end),
//     };
//     let end_clone = end.clone();

//     let mut visited = HashMap::new();
//     visited.insert(start_state.pos.clone(), start_state.clone());

//     let mut heap = BinaryHeap::new();
//     heap.push(start_state);

//     'outer: while let Some(state) = heap.pop() {
//         let mut try_neighbor = |neighbor: V| {
//             let new_pos = &state.pos + &neighbor;
//             let new_time = state.time + 1;
//             let new_dist = new_pos.dist(&end);
//             let new_state = State {
//                 pos: new_pos.clone(),
//                 time: new_time,
//                 dist: new_dist,
//             };

//             if new_pos == end {
//                 visited.insert(new_state.pos.clone(), new_state.clone());
//                 return Some(new_pos);
//             }

//             if maps[new_time.rem_euclid(maps.len())].get(&new_pos) == Some(&Cell::Empty) {
//                 if let Some(old_state) = visited.get_mut(&new_pos) {
//                     // `>` as in better
//                     if new_state > *old_state {
//                         *old_state = new_state.clone();
//                         heap.push(new_state);
//                         return Some(new_pos);
//                     }
//                 } else {
//                     visited.insert(new_state.pos.clone(), new_state.clone());
//                     heap.push(new_state);
//                     return Some(new_pos);
//                 }
//             }
//             return None;
//         };

//         let mut found_somthin = false;
//         for neighbor in &[
//             V::new(-1, 0),
//             V::new(1, 0),
//             V::new(0, 1),
//             V::new(0, -1),
//             V::new(0, 0),
//         ] {
//             match try_neighbor(neighbor.clone()) {
//                 Some(pos) => {
//                     if pos == end {
//                         break 'outer;
//                     }
//                     found_somthin = true;
//                 }
//                 None => {}
//             }
//         }
//         if !found_somthin {
//             if let Some(t) = do_a_start(state.pos, state.time + 1, end_clone.clone(), maps) {
//                 return Some(t);
//             }
//         }
//     }
//     // let mut pos = end.clone();
//     // let mut path = Vec::new();

//     // while pos != start {
//     //     path.push(pos.clone());
//     //     let best = [V::new(-1, 0), V::new(1, 0), V::new(0, 1), V::new(0, -1)]
//     //         .into_iter()
//     //         .filter_map(|n| visited.get(&(&pos + &n)))
//     //         .min_by_key(|s| s.time);
//     //     match best {
//     //         Some(b) => {
//     //             let b_pos = b.pos.clone();
//     //             visited.remove(&b_pos);
//     //             pos = b_pos.clone();
//     //         }
//     //         None => {
//     //             return None;
//     //         }
//     //     }
//     // }
//     // Some((visited.get(&end).unwrap().time, path))
//     Some(visited.get(&end).unwrap().time)
// }

fn do_breath_first(start: V, start_time: usize, end: V, maps: &Vec<Map>) -> usize {
    let mut poses = HashSet::from([start]);

    let mut time = start_time;
    while !poses.contains(&end) {
        time += 1;
        let mut new_poses = HashSet::new();
        for pos in poses {
            for neighbor in &[
                V::new(-1, 0),
                V::new(1, 0),
                V::new(0, 1),
                V::new(0, -1),
                V::new(0, 0),
            ] {
                let new_pos = &pos + &neighbor;
                if maps[time.rem_euclid(maps.len())].get(&new_pos) == Some(&Cell::Empty) {
                    new_poses.insert(new_pos);
                }
            }
        }
        poses = new_poses;
    }
    time
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    let mut blizzards = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for (y, l) in input.lines().enumerate() {
        width = (l.len() - 2) as i32;
        for (x, c) in l.chars().enumerate() {
            let dir = match c {
                '^' => Some(Dir::Up),
                '>' => Some(Dir::Right),
                '<' => Some(Dir::Left),
                'v' => Some(Dir::Down),
                _ => None,
            };
            if let Some(dir) = dir {
                blizzards.push(Blizzard {
                    pos: V::new((x - 1) as i32, (y - 1) as i32),
                    dir,
                });
            }
        }
        height += 1;
    }

    height -= 2;

    let loop_size = lcm(width, height);
    let base_map = Map::new((width + 2) as usize, (height + 2) as usize);

    let mut maps = Vec::new();
    for _ in 0..loop_size {
        let mut new_map = base_map.clone();
        for blizzard in &mut blizzards {
            *new_map.get_mut(&(V::new(1, 1) + &blizzard.pos)).unwrap() = Cell::Filled;
            blizzard.pos += blizzard.dir.to_v();
            blizzard.pos.x = blizzard.pos.x.rem_euclid(width);
            blizzard.pos.y = blizzard.pos.y.rem_euclid(height);
        }
        maps.push(new_map);
    }
    let mut assert_map = base_map.clone();
    for blizzard in &blizzards {
        *assert_map.get_mut(&(V::new(1, 1) + &blizzard.pos)).unwrap() = Cell::Filled;
    }
    assert_eq!(maps[0], assert_map);

    // let time = do_a_start(V::new(1, 0), 0, V::new(width, height + 1), &maps).unwrap();

    // let mut map_string = maps[0]
    //     .to_string()
    //     .lines()
    //     .map(|l| l.chars().collect::<Vec<_>>())
    //     .collect::<Vec<_>>();
    // for p in &path {
    //     map_string[p.y as usize][p.x as usize] = 'O';
    // }
    // println!(
    //     "{}",
    //     map_string
    //         .into_iter()
    //         .map(|l| l.iter().collect::<String>())
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // );
    let start = V::new(1, 0);
    let end = V::new(width, height + 1);

    let first_time = do_breath_first(start.clone(), 0, end.clone(), &maps);
    let second_time = do_breath_first(end.clone(), first_time, start.clone(), &maps);
    let last_time = do_breath_first(start, second_time, end, &maps);

    dbg!(last_time);
}
