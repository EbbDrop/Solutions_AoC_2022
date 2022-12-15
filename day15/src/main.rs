use std::{
    collections::HashSet,
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

    fn dist(&self, end: &V) -> u32 {
        self.x.abs_diff(end.x) + self.y.abs_diff(end.y)
    }
}

impl FromStr for V {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (xs, ys) = s.split_once(", y=").ok_or_else(|| ())?;
        let xs = xs.strip_prefix("x=").ok_or_else(|| ())?;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Sensor {
    pos: V,
    dist: i32,
    beacon: V,
}

fn main() {
    // let input = include_str!("./example.txt");
    // let max_s = 20;
    let input = include_str!("./input.txt");
    let max_s = 4000000;

    let mut sensors = Vec::new();

    let mut know_beacons = HashSet::new();

    for l in input.lines() {
        let l = l.strip_prefix("Sensor at ").unwrap();
        let (sensor_s, beacon_s) = l.split_once(": closest beacon is at ").unwrap();

        let sensor_pos: V = sensor_s.parse().unwrap();
        let beacon_pos: V = beacon_s.parse().unwrap();

        know_beacons.insert(beacon_pos.clone());

        let dist = sensor_pos.dist(&beacon_pos) as i32;

        let sensor = Sensor {
            pos: sensor_pos,
            dist,
            beacon: beacon_pos,
        };
        sensors.push(sensor);
    }

    'xs: for x in 0..max_s {
        let mut y = 0;
        'ys: while y < max_s {
            let s_pos = V::new(x, y);

            for s in &sensors {
                if s_pos.dist(&s.pos) as i32 <= s.dist {
                    y = s.pos.y + (s.dist - s_pos.x.abs_diff(s.pos.x) as i32 + 1);
                    continue 'ys;
                }
            }

            if know_beacons.contains(&s_pos) {
                y += 1;
                continue 'ys;
            }

            println!("{}, {}: {}", x, y, x as u64 * 4000000 + y as u64);
            break 'xs;
        }

        if x % 1024 == 0 {
            println!("{x} / {max_s}");
        }
    }
}
