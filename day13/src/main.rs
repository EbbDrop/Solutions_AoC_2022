use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Value(u64),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Value(v), Item::Value(o_v)) => v.cmp(o_v),
            (Item::List(l), Item::List(o_l)) => l.cmp(o_l),
            (Item::Value(_), Item::List(o_l)) => (vec![self.clone()]).cmp(o_l),
            (Item::List(l), Item::Value(_)) => l.cmp(&vec![other.clone()]),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Item {
    fn parse(it: &mut Peekable<impl Iterator<Item = char>>) -> Self {
        let c = it.next().unwrap();

        if c == '[' {
            if it.peek().unwrap() == &']' {
                it.next().unwrap();
                Self::List(Vec::new())
            } else {
                let mut list = Vec::new();
                loop {
                    let item = Self::parse(it);
                    list.push(item);
                    if it.next().unwrap() == ']' {
                        break;
                    }
                }
                Self::List(list)
            }
        } else {
            let mut s = c.to_string();
            loop {
                let c = it.peek().unwrap();
                if !c.is_numeric() {
                    break;
                }
                s.push(it.next().unwrap());
            }
            Self::Value(s.parse().unwrap())
        }
    }
}

fn main() {
    let mut input = include_str!("./input.txt").chars().peekable();
    // let mut input = include_str!("./example.txt").chars().peekable();

    let mut all = Vec::new();
    loop {
        let left = Item::parse(&mut input);
        assert!(input.next() == Some('\n'));
        let right = Item::parse(&mut input);
        all.push(left);
        all.push(right);
        if input.next() != Some('\n') {
            break;
        }
        if input.next() != Some('\n') {
            break;
        }
    }
    all.push(Item::List(vec![Item::List(vec![Item::Value(2)])]));
    all.push(Item::List(vec![Item::List(vec![Item::Value(6)])]));
    all.sort();

    for (i, item) in all.iter().enumerate() {
        if item == &Item::List(vec![Item::List(vec![Item::Value(2)])]) {
            dbg!(i + 1);
        }
        if item == &Item::List(vec![Item::List(vec![Item::Value(6)])]) {
            dbg!(i + 1);
        }
    }
}
