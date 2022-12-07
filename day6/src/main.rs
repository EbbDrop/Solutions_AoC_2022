use std::collections::{HashSet, VecDeque};

fn main() {
    let s = include_str!("./input.txt");
    let mut chars = s.chars();

    let mut last = VecDeque::new();
    for _ in 0..14 {
        last.push_back(chars.next().unwrap());
    }

    let mut i = 14;
    for c in chars {
        if last.iter().collect::<HashSet<&char>>().len() == 14 {
            break;
        }
        last.pop_front();
        last.push_back(c);
        i += 1;
    }
    dbg!(i);
}
