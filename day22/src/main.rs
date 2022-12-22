use std::{
    collections::HashMap,
    fmt::Display,
    iter::repeat,
    ops::{Add, AddAssign},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Heading {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    L,
    R,
}

impl Heading {
    fn to_v(&self) -> V {
        match self {
            Heading::Right => V::new(1, 0),
            Heading::Down => V::new(0, 1),
            Heading::Left => V::new(-1, 0),
            Heading::Up => V::new(0, -1),
        }
    }

    fn add_dir(&mut self, d: &Dir) {
        let (if_l, if_r) = match self {
            Heading::Right => (Heading::Up, Heading::Down),
            Heading::Down => (Heading::Right, Heading::Left),
            Heading::Left => (Heading::Down, Heading::Up),
            Heading::Up => (Heading::Left, Heading::Right),
        };
        *self = match d {
            Dir::L => if_l,
            Dir::R => if_r,
        };
    }

    fn to_char(&mut self) -> char {
        match self {
            Heading::Right => '>',
            Heading::Down => 'v',
            Heading::Left => '<',
            Heading::Up => '^',
        }
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

    fn wrap(&mut self, size_x: i32, size_y: i32) {
        self.x = self.x.rem_euclid(size_x);
        self.y = self.y.rem_euclid(size_y);
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
    Open,
    Wall,
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    map: Vec<Vec<Option<Cell>>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.map {
            let s = line
                .iter()
                .map(|c| match c {
                    Some(Cell::Open) => '.',
                    Some(Cell::Wall) => '#',
                    None => ' ',
                })
                .collect::<String>();
            writeln!(f, "{s}")?;
        }
        Ok(())
    }
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        let mut map = Vec::new();
        for _ in 0..height {
            map.push(vec![None; width]);
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
            .and_then(|o| o.as_ref())
    }

    fn get_mut<'a>(&'a mut self, v: &V) -> Option<&'a mut Option<Cell>> {
        if v.y < 0 || v.x < 0 {
            return None;
        }
        self.map
            .get_mut(v.y as usize)
            .and_then(|l| l.get_mut(v.x as usize))
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Move(i32),
    Turn(Dir),
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
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");
    let (input_map, input_path) = input.split_once("\n\n").unwrap();

    let mut input_lines = Vec::new();
    let mut width = 0;
    for l in input_map.lines() {
        width = width.max(l.len());
        input_lines.push(l);
    }

    let height = input_lines.len();
    let face_size = (width / 3) as i32;
    let mut map = Map::new(width, height);
    for (y, l) in input_lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let c = match c {
                '.' => Some(Cell::Open),
                '#' => Some(Cell::Wall),
                ' ' => None,
                _ => unreachable!(),
            };

            let v = V::new(x as i32, y as i32);
            *map.get_mut(&v).unwrap() = c;
        }
    }

    let mut path = Vec::new();
    for c in input_path.split_inclusive(['R', 'L']) {
        let amount = c.trim().strip_suffix(['R', 'L']).unwrap_or(c.trim());
        let dir = c.matches(['R', 'L']).next();

        path.push(Instruction::Move(amount.parse().unwrap()));

        if let Some(d) = dir {
            let d = match d {
                "R" => Dir::R,
                "L" => Dir::L,
                _ => unreachable!(),
            };
            path.push(Instruction::Turn(d));
        }
    }

    let mut start_x = 0;
    while map.get(&V::new(start_x, 0)) != Some(&Cell::Open) {
        start_x += 1;
    }

    let mut pos = V::new(start_x, 0);
    let mut heading = Heading::Right;

    let mut d = Display2D::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let c = match map.get(&V::new(x as i32, y as i32)) {
                Some(Cell::Open) => '.',
                Some(Cell::Wall) => '#',
                None => ' ',
            };
            d.draw(x as i32, y as i32, c);
        }
    }

    // let conections = HashMap::from([
    //     ((2, 0, Heading::Up), (0, 1, Heading::Down, true)),
    //     ((2, 0, Heading::Left), (1, 1, Heading::Down, false)),
    //     ((2, 0, Heading::Right), (3, 2, Heading::Left, true)),
    //     ((0, 1, Heading::Up), (2, 0, Heading::Down, true)),
    //     ((0, 1, Heading::Left), (3, 2, Heading::Up, true)),
    //     ((0, 1, Heading::Down), (2, 2, Heading::Up, true)),
    //     ((1, 1, Heading::Up), (2, 0, Heading::Right, false)),
    //     ((1, 1, Heading::Down), (2, 2, Heading::Right, true)),
    //     ((2, 1, Heading::Right), (3, 2, Heading::Down, true)),
    //     ((2, 2, Heading::Left), (1, 1, Heading::Up, true)),
    //     ((2, 2, Heading::Down), (0, 1, Heading::Up, true)),
    //     ((3, 2, Heading::Up), (2, 1, Heading::Left, true)),
    //     ((3, 2, Heading::Right), (2, 0, Heading::Left, true)),
    //     ((3, 2, Heading::Down), (0, 1, Heading::Right, true)),
    // ]);

    #[rustfmt::skip]
    let conections = HashMap::from([
        ((1i32, 0i32, Heading::Left), (0i32, 2i32, Heading::Right, true)),
        ((0, 2, Heading::Left), (1, 0, Heading::Right, true)),

        ((1, 0, Heading::Up), (0, 3, Heading::Right, false)),
        ((0, 3, Heading::Left), (1, 0, Heading::Down, false)),


        ((2, 0, Heading::Up), (0, 3, Heading::Up, false)),
        ((0, 3, Heading::Down), (2, 0, Heading::Down, false)),

        ((2, 0, Heading::Right), (1, 2, Heading::Left, true)),
        ((1, 2, Heading::Right), (2, 0, Heading::Left, true)),

        ((2, 0, Heading::Down), (1, 1, Heading::Left, false)),
        ((1, 1, Heading::Right), (2, 0, Heading::Up, false)),


        ((1, 1, Heading::Left), (0, 2, Heading::Down, false)),
        ((0, 2, Heading::Up), (1, 1, Heading::Right, false)),


        ((1, 2, Heading::Down), (0, 3, Heading::Left, false)),
        ((0, 3, Heading::Right), (1, 2, Heading::Up, false)),
    ]);

    // let numbers = [[0, 1, 2], [0, 3, 0], [4, 5, 0], [6, 0, 0]];
    // for ((x, y, _), (tx, ty, _, _)) in conections.iter() {
    //     println!("{} -> {}", numbers[*y][*x], numbers[*ty][*tx]);
    // }

    for i in path {
        match i {
            Instruction::Move(a) => {
                for _ in 0..a {
                    d.draw(pos.x, pos.y, heading.to_char());
                    let move_v = heading.to_v();
                    match map.get(&(pos.clone() + &move_v)) {
                        Some(Cell::Open) => pos += move_v,
                        Some(Cell::Wall) => break,
                        None => {
                            let (face_x, face_y) = (pos.x / face_size, pos.y / face_size);
                            let Some((new_face_x, new_face_y, new_heading, invert)) =
                                conections.get(&(face_x, face_y, heading.clone())) else {
                                panic!("face_x: {face_x}, face_y: {face_y}, h: {:?}", heading);
                            };

                            let mut movable = match heading {
                                Heading::Right | Heading::Left => pos.y - face_y * face_size,
                                Heading::Down | Heading::Up => pos.x - face_x * face_size,
                            };
                            if *invert {
                                movable = face_size - movable - 1;
                            }
                            dbg!(&movable);

                            let new_pos = match new_heading {
                                Heading::Right => {
                                    V::new(new_face_x * face_size, new_face_y * face_size + movable)
                                }
                                Heading::Down => {
                                    V::new(new_face_x * face_size + movable, new_face_y * face_size)
                                }
                                Heading::Left => V::new(
                                    new_face_x * face_size + face_size - 1,
                                    new_face_y * face_size + movable,
                                ),
                                Heading::Up => V::new(
                                    new_face_x * face_size + movable,
                                    new_face_y * face_size + face_size - 1,
                                ),
                            };
                            match map.get(&new_pos) {
                                Some(Cell::Open) => {
                                    pos = new_pos;
                                    heading = new_heading.clone();
                                }
                                Some(Cell::Wall) => continue,
                                None => {} // unreachable!(),
                            }
                        }
                    }
                }
            }
            Instruction::Turn(d) => heading.add_dir(&d),
        }
    }
    println!("{}", d);
    dbg!((pos.y + 1) * 1000 + (pos.x + 1) * 4 + heading as i32);
}
