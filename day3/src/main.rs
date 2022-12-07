#![feature(iter_array_chunks)]
use std::collections::HashSet;

fn to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unimplemented!(),
    }
}

// fn main() {
//     let input = include_str!("./input.txt").lines();

//     let mut score = 0;

//     for l in input {
//         let (a, b) = l.split_at(l.len() / 2);

//         for item in a.chars() {
//             if b.contains(item) {
//                 score += to_priority(item);
//                 break;
//             }
//         }
//     }

//     dbg!(score);
// }

fn main() {
    let input = include_str!("./input.txt").lines();

    let mut score = 0;

    for [a, b, c] in input.array_chunks::<3>() {
        let set_a: HashSet<_> = a.chars().collect();
        let set_b: HashSet<_> = b.chars().collect();
        let set_c: HashSet<_> = c.chars().collect();

        let set_a: HashSet<char> = set_a.intersection(&set_b).map(|c| *c).collect();
        let mut com = set_a.intersection(&set_c);

        let item = com.next().unwrap();

        score += to_priority(*item);
    }

    dbg!(score);
}
