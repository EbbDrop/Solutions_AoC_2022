fn main() {
    let (crates, moves) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut staks: Vec<Vec<char>> = vec![vec![]; 9];

    for line in crates.lines().rev().skip(1) {
        for i in 0..9 {
            let c = line.chars().nth(1 + i * 4).unwrap();
            if c != ' ' {
                staks[i].push(c);
            }
        }
    }

    for line in moves.lines() {
        let line = line.strip_prefix("move ").unwrap();
        let (amount, rest) = line.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();

        let amount: usize = amount.parse().unwrap();
        let from: usize = from.parse().unwrap();
        let to: usize = to.parse().unwrap();

        let len = staks[from - 1].len();
        let moves = staks[from - 1].spit_off(len - amount);
        staks[to - 1].extend_from_slice(&moves);
    }

    for s in staks {
        print!("{}", s.last().unwrap())
    }
    println!();
}
