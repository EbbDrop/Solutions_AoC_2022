use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct V {
    x: i64,
    y: i64,
}

impl V {
    fn new(x: i64, y: i64) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn to_v(&self) -> V {
        match self {
            Jet::Left => V::new(-1, 0),
            Jet::Right => V::new(1, 0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape(Vec<V>);

impl Shape {
    fn shift(&mut self, v: &V) {
        for p in self.0.iter_mut() {
            *p += v;
        }
    }

    fn with_shift<'a>(&'a self, v: &'a V) -> impl Iterator<Item = V> + '_ {
        self.0.iter().map(|p| p.clone() + v.clone())
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        let height = lines.len();

        let mut points = Vec::new();

        for (i, l) in lines.into_iter().enumerate() {
            let y = height - i - 1;
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    points.push(V::new(x as i64, y as i64));
                }
            }
        }

        Ok(Self(points))
    }
}

#[derive(Debug, Clone)]
struct Display2D(Vec<String>);

impl Display2D {
    fn new(w: usize, h: usize) -> Self {
        let mut d = Vec::with_capacity(h);

        for _ in 0..h {
            d.push(".".repeat(w))
        }

        Display2D(d)
    }

    fn draw(&mut self, x: i64, y: i64, c: char) {
        if y >= 0 && y < self.0.len() as i64 {
            let l = &mut self.0[y as usize];
            if x >= 0 && x < l.len() as i64 {
                l.replace_range(x as usize..x as usize + 1, &c.to_string());
            }
        }
    }
}

impl Display for Display2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.0.iter().rev() {
            writeln!(f, "|{}|", l)?;
        }
        writeln!(f, "+{}+", "-".repeat(self.0[0].len()))?;
        Ok(())
    }
}

fn fall(
    mut jet_index: usize,
    max_shapes: Option<u64>,
    shapes: &Vec<Shape>,
    jets: &Vec<Jet>,
) -> (i64, u64, usize) {
    let mut shape_index = 0;
    let mut total_shapes = 0;

    let mut height = 0;
    let mut fallen: HashMap<V, usize> = HashMap::new();
    loop {
        let mut shape = shapes[shape_index].clone();
        total_shapes += 1;
        shape.shift(&V::new(2, height + 3));

        let mut try_shift = |v: &V| -> bool {
            let mut fits = true;
            for p in shape.with_shift(v) {
                if p.x < 0 || p.x >= 7 || p.y < 0 || fallen.contains_key(&p) {
                    fits = false;
                    break;
                }
            }
            if fits {
                shape.shift(v);
            }
            fits
        };

        loop {
            let jet = jets[jet_index].clone();
            jet_index = (jet_index + 1) % jets.len();

            try_shift(&jet.to_v());

            if !try_shift(&V::new(0, -1)) {
                for p in shape.0 {
                    height = height.max(p.y + 1);
                    fallen.insert(p, shape_index);
                }
                break;
            }
        }
        shape_index = (shape_index + 1) % shapes.len();
        if let Some(max_shapes) = max_shapes {
            if max_shapes == total_shapes {
                break;
            }
        }
        if shape_index == 0 {
            // Value that gives a nice top surface for my input
            if jet_index == 4783 {
                break;
            }
        }
    }

    let mut d = Display2D::new(7, (8) as usize);
    for (p, i) in &fallen {
        if p.y > height - 4 {
            d.draw(p.x, p.y - height + 8, i.to_string().chars().next().unwrap());
        }
        if p.y < 4 {
            d.draw(p.x, p.y, i.to_string().chars().next().unwrap());
        }
    }
    println!("{}", d);
    dbg!(shape_index);
    dbg!(jet_index);

    (height, total_shapes, jet_index)
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    let mut jets = Vec::new();
    for c in input.trim().chars() {
        let j = match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unreachable!("{}", c),
        };
        jets.push(j);
    }

    let mut shapes = Vec::new();
    shapes.push(Shape::from_str("####").unwrap());
    shapes.push(
        Shape::from_str(
            ".#.
###
.#.",
        )
        .unwrap(),
    );
    shapes.push(
        Shape::from_str(
            r#"..#
..#
###"#,
        )
        .unwrap(),
    );
    shapes.push(
        Shape::from_str(
            r#"#
#
#
#"#,
        )
        .unwrap(),
    );
    shapes.push(
        Shape::from_str(
            r#"##
##"#,
        )
        .unwrap(),
    );

    let mut total_height = 0;
    let mut total_shapes = 1000000000000;

    let (h, s, j) = dbg!(fall(0, None, &shapes, &jets));
    total_shapes -= s;
    total_height += h;

    let (h, s, j) = dbg!(fall(j, None, &shapes, &jets));

    let repeats = dbg!(total_shapes / s);
    total_height += h * repeats as i64;
    total_shapes -= s * repeats;

    let (h, s, _) = dbg!(fall(j, Some(total_shapes), &shapes, &jets));

    total_height += h;
    total_shapes -= s;

    dbg!(total_shapes);

    dbg!(total_height);
}
