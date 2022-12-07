use std::{
    ops::{RangeBounds, RangeInclusive},
    str::FromStr,
};

struct Pair {
    first: RangeInclusive<i64>,
    second: RangeInclusive<i64>,
}

impl Pair {
    pub fn contains(&self) -> bool {
        (self.first.contains(self.second.start()) || self.first.contains(self.second.end()))
            || (self.second.contains(self.first.start()) || self.second.contains(self.first.end()))
    }
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (f, s) = s.split_once(",").ok_or("No ,")?;

        let (ff, fs) = f.split_once("-").ok_or("No - at first")?;
        let (sf, ss) = s.split_once("-").ok_or("No - at second")?;

        let ff: i64 = ff.parse::<i64>().map_err(|e| e.to_string())?;
        let fs: i64 = fs.parse::<i64>().map_err(|e| e.to_string())?;
        let sf: i64 = sf.parse::<i64>().map_err(|e| e.to_string())?;
        let ss: i64 = ss.parse::<i64>().map_err(|e| e.to_string())?;

        Ok(Self {
            first: ff..=fs,
            second: sf..=ss,
        })
    }
}

fn main() {
    let lines = include_str!("./input.txt").lines();

    let pairs: Vec<Pair> = lines.map(|l| l.parse::<Pair>().ok()).flatten().collect();

    dbg!(pairs.iter().filter(|p| p.contains()).count());
}
