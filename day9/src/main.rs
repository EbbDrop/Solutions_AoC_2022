use std::{
    collections::HashSet,
    fmt::Display,
    iter::repeat,
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

#[derive(Debug, Clone)]
struct Display2D(Vec<String>);

impl Display2D {
    fn new(w: usize, h: usize) -> Self {
        let mut d = Vec::with_capacity(h);

        for _ in 0..h {
            d.push(repeat('.').take(w).collect())
        }

        Display2D(d)
    }

    fn draw(&mut self, v: &V, c: char) {
        if v.y >= 0 && v.y < self.0.len() as i32 {
            let l = &mut self.0[v.y as usize];
            if v.x >= 0 && v.x < l.len() as i32 {
                l.replace_range(v.x as usize..v.x as usize + 1, &c.to_string());
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
    // let moves = include_str!("./example.txt").lines();
    let moves = include_str!("./input.txt").lines();
    let moves = moves
        .map(|l| l.split_once(" ").unwrap())
        .map(|(m, a)| {
            let m = match m {
                "R" => V::new(1, 0),
                "D" => V::new(0, 1),
                "L" => V::new(-1, 0),
                "U" => V::new(0, -1),
                _ => panic!("`{}` isen't a move", m),
            };
            (m, a.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut rope = vec![V::new(8, 8); 10];

    let mut d = Display2D::new(16, 16);

    let mut been = HashSet::new();

    for (mov, amount) in &moves {
        for _ in 0..*amount {
            rope[0] += mov;

            for i in 0..9 {
                let (head, tail) = rope.split_at_mut(i + 1);
                let head = &mut head[head.len() - 1];
                let tail = &mut tail[0];

                if (head.x - tail.x).abs() >= 2 || (head.y - tail.y).abs() >= 2 {
                    // match ((head.x - tail.x).abs(), (head.y - tail.y).abs()) {
                    //     (2, 0) => tail.x += (head.x - tail.x) / 2,
                    //     (0, 2) => tail.y += (head.y - tail.y) / 2,
                    //     (2, 1) => {
                    //         tail.x += (head.x - tail.x) / 2;
                    //         tail.y += head.y - tail.y;
                    //     }
                    //     (1, 2) => {
                    //         tail.x += head.x - tail.x;
                    //         tail.y += (head.y - tail.y) / 2;
                    //     }
                    //     (2, 2) => {
                    //         tail.x += (head.x - tail.x) / 2;
                    //         tail.y += (head.y - tail.y) / 2;
                    //     }
                    //     _ => {
                    //         dbg!(head.x - tail.x, head.y - tail.y, i);
                    //         unreachable!()
                    //     }
                    // }
                    tail.x += (head.x - tail.x).signum();
                    tail.y += (head.y - tail.y).signum();
                }
            }

            been.insert(rope[9].clone());
        }

        // d.clear();
        // for i in 0..9 {
        //     d.draw(&rope[i], i.to_string().chars().next().unwrap());
        // }
        // println!("{}", &d);
    }

    dbg!(been.len());
}
